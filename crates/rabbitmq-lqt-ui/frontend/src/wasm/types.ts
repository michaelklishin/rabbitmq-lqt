export interface ValidationResult {
  valid: boolean
  error_message: string | null
  error_position: number | null
  suggestions: string[]
}

export interface FieldInfo {
  name: string
  aliases: string[]
  description: string
  example_values: string[]
}

export interface OperatorInfo {
  symbol: string
  aliases: string[]
  name: string
  description: string
}

export interface PipelineStageInfo {
  name: string
  aliases: string[]
  syntax: string
  description: string
}

export interface DurationUnitInfo {
  suffix: string
  name: string
  example: string
}

export interface PresetInfo {
  name: string
  description: string
  query_string: string
}

export interface AutocompleteData {
  severities: string[]
  subsystems: string[]
  labels: string[]
  fields: FieldInfo[]
  operators: OperatorInfo[]
  pipeline_stages: PipelineStageInfo[]
  duration_units: DurationUnitInfo[]
  presets: PresetInfo[]
  special_filters: string[]
}
