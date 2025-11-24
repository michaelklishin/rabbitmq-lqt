import { useState } from 'react'
import { FileMetadataResponse } from '../api/client'
import { formatDate } from '../utils/dateFormat'

interface MetadataHeaderProps {
  fileMetadata: FileMetadataResponse[] | undefined
  onViewFullMetadata: () => void
}

export function MetadataHeader({ fileMetadata, onViewFullMetadata }: MetadataHeaderProps) {
  const [isExpanded, setIsExpanded] = useState(true)

  if (!fileMetadata || fileMetadata.length === 0) {
    return null
  }

  const allVersions = new Set(
    fileMetadata
      .map((f) => f.rabbitmq_version)
      .filter((v): v is string => v !== null)
  )

  const allPlugins = Array.from(
    new Set(fileMetadata.flatMap((f) => f.enabled_plugins))
  ).sort()

  const oldestEntry = fileMetadata
    .map((f) => f.oldest_entry_at)
    .filter((d): d is string => d !== null)
    .sort()[0]

  const newestEntry = fileMetadata
    .map((f) => f.most_recent_entry_at)
    .filter((d): d is string => d !== null)
    .sort()
    .reverse()[0]

  return (
    <div className="bg-white shadow-sm border border-gray-200 rounded-lg mb-4">
      <button
        onClick={() => setIsExpanded(!isExpanded)}
        className="w-full px-4 py-3 flex items-center justify-between text-left hover:bg-gray-50 transition-colors"
      >
        <h3 className="text-sm font-semibold text-gray-900">File Set Overview</h3>
        <svg
          className={`w-5 h-5 text-gray-500 transition-transform ${
            isExpanded ? 'rotate-180' : ''
          }`}
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M19 9l-7 7-7-7"
          />
        </svg>
      </button>

      {isExpanded && (
        <div className="px-4 pb-4 space-y-3 border-t border-gray-100">
          {allVersions.size > 0 && (
            <div className="pt-3">
              <dt className="text-xs font-medium text-gray-500">RabbitMQ Version</dt>
              <dd className="mt-1 text-sm text-gray-900">
                {Array.from(allVersions).join(', ')}
              </dd>
            </div>
          )}

          {oldestEntry && newestEntry && (
            <div>
              <dt className="text-xs font-medium text-gray-500">Time Range</dt>
              <dd className="mt-1 text-sm text-gray-900">
                {formatDate(oldestEntry)} → {formatDate(newestEntry)}
              </dd>
            </div>
          )}

          <div>
            <dt className="text-xs font-medium text-gray-500">Total Entries / Lines</dt>
            <dd className="mt-1 text-sm text-gray-900">
              {fileMetadata.reduce((sum, f) => sum + f.total_entries, 0).toLocaleString()} /{' '}
              {fileMetadata.reduce((sum, f) => sum + f.total_lines, 0).toLocaleString()}
            </dd>
          </div>

          {allPlugins.length > 0 && (
            <div>
              <dt className="text-xs font-medium text-gray-500">
                Enabled Plugins ({allPlugins.length})
              </dt>
              <dd className="mt-2 flex flex-wrap gap-1">
                {allPlugins.slice(0, 10).map((plugin) => (
                  <span
                    key={plugin}
                    className="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-orange-100 text-orange-800"
                  >
                    {plugin}
                  </span>
                ))}
                {allPlugins.length > 10 && (
                  <span className="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-gray-100 text-gray-600">
                    +{allPlugins.length - 10} more
                  </span>
                )}
              </dd>
            </div>
          )}

          <div className="pt-2">
            <button
              onClick={onViewFullMetadata}
              className="text-xs text-blue-600 hover:text-blue-800 font-medium"
            >
              View Full Metadata →
            </button>
          </div>
        </div>
      )}
    </div>
  )
}
