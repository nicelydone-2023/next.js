import type { GetDynamicParamFromSegment } from '../../server/app-render/app-render'
import type { LoaderTree } from '../../server/lib/app-dir-module'

import React from 'react'
import {
  AppleWebAppMeta,
  FormatDetectionMeta,
  ItunesMeta,
  BasicMetadata,
  VerificationMeta,
} from './generate/basic'
import { AlternatesMetadata } from './generate/alternate'
import {
  OpenGraphMetadata,
  TwitterMetadata,
  AppLinksMeta,
} from './generate/opengraph'
import { IconsMetadata } from './generate/icons'
import { accumulateMetadata, resolveMetadata } from './resolve-metadata'
import { MetaFilter } from './generate/meta'
import { ClientMetadataWrap } from './metadata-client'

// Generate the actual React elements from the resolved metadata.
export async function MetadataTree({
  tree,
  pathname,
  searchParams,
  getDynamicParamFromSegment,
  requestId,
}: {
  tree: LoaderTree
  pathname: string
  searchParams: { [key: string]: any }
  getDynamicParamFromSegment: GetDynamicParamFromSegment
  requestId: string
}) {
  const metadataContext = {
    pathname,
  }
  const resolvedMetadata = await resolveMetadata({
    tree,
    parentParams: {},
    metadataItems: [],
    searchParams,
    getDynamicParamFromSegment,
  })
  const metadata = await accumulateMetadata(resolvedMetadata, metadataContext)

  const elements = MetaFilter([
    BasicMetadata({ metadata }),
    AlternatesMetadata({ alternates: metadata.alternates }),
    ItunesMeta({ itunes: metadata.itunes }),
    FormatDetectionMeta({ formatDetection: metadata.formatDetection }),
    VerificationMeta({ verification: metadata.verification }),
    AppleWebAppMeta({ appleWebApp: metadata.appleWebApp }),
    OpenGraphMetadata({ openGraph: metadata.openGraph }),
    TwitterMetadata({ twitter: metadata.twitter }),
    AppLinksMeta({ appLinks: metadata.appLinks }),
    IconsMetadata({ icons: metadata.icons }),
  ])

  // const children =

  return (
    <ClientMetadataWrap key={requestId}>
      {elements.map((el, index) => {
        return React.cloneElement(el as React.ReactElement, { key: index })
      })}
    </ClientMetadataWrap>
  )
}
