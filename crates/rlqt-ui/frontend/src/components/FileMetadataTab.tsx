import { FileMetadataResponse } from '../api/client'
import { formatDate } from '../utils/dateFormat'

interface FileMetadataTabProps {
  fileMetadata: FileMetadataResponse[] | undefined
}

function extractFilename(filePath: string): string {
  const parts = filePath.split('/')
  return parts[parts.length - 1] || filePath
}

export function FileMetadataTab({ fileMetadata }: FileMetadataTabProps) {
  if (!fileMetadata || fileMetadata.length === 0) {
    return (
      <div className="bg-white shadow-sm border border-gray-200 rounded-lg p-8 text-center text-gray-500">
        No file metadata found in database
      </div>
    )
  }

  return (
    <div className="space-y-6">
      {fileMetadata.map((metadata, index) => (
        <div
          key={metadata.file_path}
          className="bg-white shadow-sm border border-gray-200 rounded-lg p-6"
        >
          {index > 0 && <div className="mb-4" />}

          <h3 className="text-lg font-semibold text-gray-900 mb-4">
            {extractFilename(metadata.file_path)}
          </h3>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {metadata.rabbitmq_version && (
              <div>
                <dt className="text-sm font-medium text-gray-500">RabbitMQ Version</dt>
                <dd className="mt-1 text-sm text-gray-900">{metadata.rabbitmq_version}</dd>
              </div>
            )}

            {metadata.erlang_version && (
              <div>
                <dt className="text-sm font-medium text-gray-500">Erlang Version</dt>
                <dd className="mt-1 text-sm text-gray-900">{metadata.erlang_version}</dd>
              </div>
            )}

            {metadata.tls_library && (
              <div>
                <dt className="text-sm font-medium text-gray-500">TLS Library</dt>
                <dd className="mt-1 text-sm text-gray-900">{metadata.tls_library}</dd>
              </div>
            )}

            {metadata.oldest_entry_at && (
              <div>
                <dt className="text-sm font-medium text-gray-500">Oldest Entry</dt>
                <dd className="mt-1 text-sm text-gray-900">
                  {formatDate(metadata.oldest_entry_at)}
                </dd>
              </div>
            )}

            {metadata.most_recent_entry_at && (
              <div>
                <dt className="text-sm font-medium text-gray-500">Most Recent Entry</dt>
                <dd className="mt-1 text-sm text-gray-900">
                  {formatDate(metadata.most_recent_entry_at)}
                </dd>
              </div>
            )}

            <div>
              <dt className="text-sm font-medium text-gray-500">Total Lines</dt>
              <dd className="mt-1 text-sm text-gray-900">{metadata.total_lines.toLocaleString()}</dd>
            </div>

            <div>
              <dt className="text-sm font-medium text-gray-500">Total Entries</dt>
              <dd className="mt-1 text-sm text-gray-900">{metadata.total_entries.toLocaleString()}</dd>
            </div>
          </div>

          {metadata.nodes.length > 0 && (
            <div className="mt-4">
              <dt className="text-sm font-medium text-gray-500">Nodes</dt>
              <dd className="mt-2 flex flex-wrap gap-2">
                {metadata.nodes.map((node) => (
                  <span
                    key={node}
                    className="inline-flex items-center px-2.5 py-0.5 rounded-md text-sm font-medium bg-blue-100 text-blue-800"
                  >
                    {node}
                  </span>
                ))}
              </dd>
            </div>
          )}

          {metadata.subsystems.length > 0 && (
            <div className="mt-4">
              <dt className="text-sm font-medium text-gray-500">Subsystems</dt>
              <dd className="mt-2 flex flex-wrap gap-2">
                {metadata.subsystems.map((subsystem) => (
                  <span
                    key={subsystem}
                    className="inline-flex items-center px-2.5 py-0.5 rounded-md text-sm font-medium bg-green-100 text-green-800"
                  >
                    {subsystem}
                  </span>
                ))}
              </dd>
            </div>
          )}

          {metadata.labels.length > 0 && (
            <div className="mt-4">
              <dt className="text-sm font-medium text-gray-500">Labels</dt>
              <dd className="mt-2 flex flex-wrap gap-2">
                {metadata.labels.map((label) => (
                  <span
                    key={label}
                    className="inline-flex items-center px-2.5 py-0.5 rounded-md text-sm font-medium bg-purple-100 text-purple-800"
                  >
                    {label}
                  </span>
                ))}
              </dd>
            </div>
          )}

          {metadata.enabled_plugins.length > 0 && (
            <div className="mt-4">
              <dt className="text-sm font-medium text-gray-500">
                Enabled Plugins ({metadata.enabled_plugins.length})
              </dt>
              <dd className="mt-2 flex flex-wrap gap-2">
                {metadata.enabled_plugins.map((plugin) => (
                  <span
                    key={plugin}
                    className="inline-flex items-center px-2.5 py-0.5 rounded-md text-sm font-medium bg-orange-100 text-orange-800"
                  >
                    {plugin}
                  </span>
                ))}
              </dd>
            </div>
          )}
        </div>
      ))}
    </div>
  )
}
