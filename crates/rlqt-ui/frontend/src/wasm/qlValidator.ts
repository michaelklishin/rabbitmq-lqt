import type { ValidationResult, AutocompleteData } from './types'

interface WasmModule {
  validate_query(input: string): ValidationResult
  get_autocomplete_data(): AutocompleteData
  complete_field(prefix: string): string[]
  complete_severity(prefix: string): string[]
  complete_subsystem(prefix: string): string[]
  complete_label(prefix: string): string[]
  complete_preset(prefix: string): string[]
  suggest_field(input: string): string[]
  suggest_severity(input: string): string[]
  suggest_subsystem(input: string): string[]
  suggest_label(input: string): string[]
  suggest_preset(input: string): string[]
}

let wasmModule: WasmModule | null = null
let initPromise: Promise<void> | null = null

export async function initWasm(): Promise<void> {
  if (wasmModule) return
  if (initPromise) return initPromise

  initPromise = (async () => {
    try {
      const module = await import('./pkg/rlqt_ql_wasm')
      // Initialize the WASM module. This must be called before using other exports!
      await module.default()
      wasmModule = module as unknown as WasmModule
    } catch (e) {
      console.warn('WASM module not available, validation disabled:', e)
      initPromise = null
    }
  })()

  return initPromise
}

export function isWasmAvailable(): boolean {
  return wasmModule !== null
}

export function validateQuery(input: string): ValidationResult {
  if (!wasmModule) {
    return {
      valid: true,
      error_message: null,
      error_position: null,
      suggestions: [],
    }
  }
  return wasmModule.validate_query(input)
}

export function getAutocompleteData(): AutocompleteData | null {
  if (!wasmModule) return null
  return wasmModule.get_autocomplete_data()
}

export function completeField(prefix: string): string[] {
  if (!wasmModule) return []
  return wasmModule.complete_field(prefix)
}

export function completeSeverity(prefix: string): string[] {
  if (!wasmModule) return []
  return wasmModule.complete_severity(prefix)
}

export function completeSubsystem(prefix: string): string[] {
  if (!wasmModule) return []
  return wasmModule.complete_subsystem(prefix)
}

export function completeLabel(prefix: string): string[] {
  if (!wasmModule) return []
  return wasmModule.complete_label(prefix)
}

export function completePreset(prefix: string): string[] {
  if (!wasmModule) return []
  return wasmModule.complete_preset(prefix)
}

export function suggestField(input: string): string[] {
  if (!wasmModule) return []
  return wasmModule.suggest_field(input)
}

export function suggestSeverity(input: string): string[] {
  if (!wasmModule) return []
  return wasmModule.suggest_severity(input)
}

export function suggestSubsystem(input: string): string[] {
  if (!wasmModule) return []
  return wasmModule.suggest_subsystem(input)
}

export function suggestLabel(input: string): string[] {
  if (!wasmModule) return []
  return wasmModule.suggest_label(input)
}

export function suggestPreset(input: string): string[] {
  if (!wasmModule) return []
  return wasmModule.suggest_preset(input)
}
