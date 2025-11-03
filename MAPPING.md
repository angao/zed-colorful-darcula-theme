# GoLand to Zed Theme Mapping Guide

This document explains how colors from the original GoLand/RustRover Colorful Darcula scheme (`Colorful_Darcula.icls`) have been mapped to Zed's theme system.

## Overview

Zed uses a different syntax highlighting system than JetBrains IDEs. This mapping translates GoLand's semantic tokens to Zed's syntax categories while preserving the original color scheme's intent.

## Direct Mappings

### Go Language Elements

| GoLand Token | Color | Zed Mapping | Notes |
|--------------|-------|-------------|-------|
| `GO_EXPORTED_FUNCTION` | `#ffe37b` | `syntax.function` | Applied to all public functions |
| `GO_EXPORTED_FUNCTION_CALL` | `#ffe37b` | `syntax.function` | Function call highlighting |
| `GO_LOCAL_FUNCTION` | `#ff7b00` | `syntax.function` | Currently uses same as exported* |
| `GO_LOCAL_FUNCTION_CALL` | `#ff7b00` | `syntax.function` | Currently uses same as exported* |
| `GO_METHOD_RECEIVER` | `#ffff00` | `syntax.variable.special` | Self/receiver parameters |
| `GO_FUNCTION_PARAMETER` | `#9077c6` | `syntax.variable` | Function parameters |
| `GO_LOCAL_VARIABLE` | `#6598c6` | `syntax.variable` | Local scope variables* |
| `GO_LOCAL_VARIABLE_CALL` | `#6598c6` | `syntax.variable` | Variable references* |
| `GO_IDENTIFIER` | `#9077c6` | `syntax.variable` | Generic identifiers |
| `GO_PACKAGE_EXPORTED_VARIABLE` | `#9077c6` | `syntax.variable` | Public variables |
| `GO_PACKAGE_EXPORTED_VARIABLE_CALL` | `#9077c6` | `syntax.variable` | Public variable refs |
| `GO_PACKAGE_LOCAL_VARIABLE` | `#816093` | `syntax.variable` | Package-level vars* |
| `GO_PACKAGE_LOCAL_VARIABLE_CALL` | `#816093` | `syntax.variable` | Package var refs* |
| `GO_SHADOWING_VARIABLE` | `#2ce7ac` | `syntax.variable` | Shadowed variables* |
| `GO_SCOPE_VARIABLE` | `#c6c600` | `syntax.variable` | Block scope vars* |

### Type Elements

| GoLand Token | Color | Zed Mapping | Notes |
|--------------|-------|-------------|-------|
| `GO_PACKAGE_EXPORTED_STRUCT` | `#c67f58` | `syntax.type` | Public structs |
| `GO_PACKAGE_LOCAL_STRUCT` | `#8996a5` | `syntax.type` | Private structs* |
| `GO_PACKAGE_EXPORTED_INTERFACE` | `#c6a469` | `syntax.enum` | Public interfaces |
| `GO_PACKAGE_LOCAL_INTERFACE` | `#7d91c6` | `syntax.enum` | Private interfaces* |
| `GO_TYPE_SPECIFICATION` | `#51c688` | `syntax.type` | Type definitions* |

### Constants and Fields

| GoLand Token | Color | Zed Mapping | Notes |
|--------------|-------|-------------|-------|
| `GO_LOCAL_CONSTANT` | `#9876aa` | `syntax.constant` | Local constants |
| `GO_PACKAGE_EXPORTED_CONSTANT` | `#9876aa` | `syntax.constant` | Public constants |
| `GO_PACKAGE_LOCAL_CONSTANT` | `#be93d0` | `syntax.constant` | Package constants* |
| `GO_STRUCT_EXPORTED_MEMBER` | `#5bc600` | `syntax.property` | Public struct fields* |
| `GO_STRUCT_LOCAL_MEMBER` | `#c62bff` | `syntax.property` | Private fields* |

### Other Elements

| GoLand Token | Color | Zed Mapping | Notes |
|--------------|-------|-------------|-------|
| `GO_STRING` | `#30be47` | `syntax.string` | String literals |
| `GO_COMMENT_REFERENCE` | `#a1a1a1` | `syntax.comment` | Comments |

## Zed-Specific Mappings

These are Zed syntax categories that don't have direct GoLand equivalents but use colors from the palette:

| Zed Syntax Category | Color | Source |
|---------------------|-------|--------|
| `syntax.keyword` | `#cc7832` | Darcula keyword color |
| `syntax.number` | `#6897bb` | Darcula number color |
| `syntax.boolean` | `#cc7832` | Same as keyword |
| `syntax.operator` | `#a9b7c6` | Darcula default text |
| `syntax.punctuation` | `#a9b7c6` | Darcula default text |
| `syntax.constructor` | `#ffc66d` | Darcula constructor color |
| `syntax.attribute` | `#bbb529` | Annotation/decorator color |
| `syntax.string.escape` | `#cc7832` | Escape sequence highlighting |
| `syntax.string.regex` | `#51c688` | Regex pattern color |
| `syntax.tag` | `#ffc66d` | HTML/XML tags |

## Limitations and Notes

### Semantic Highlighting Differences

**Note:** Items marked with `*` indicate colors that may not fully apply in Zed due to semantic highlighting limitations.

Zed's current syntax highlighting system (as of the theme creation) has some differences from JetBrains IDEs:

1. **Exported vs Local Distinction**: 
   - GoLand: Distinguishes between exported and local symbols by capitalization and scope
   - Zed: Uses unified categories; distinction may require language server support
   - **Impact**: `GO_LOCAL_FUNCTION` and `GO_EXPORTED_FUNCTION` both map to `#ffe37b`

2. **Variable Scope Tracking**:
   - GoLand: Tracks package-level, local, and shadowed variables separately
   - Zed: Generally uses single `variable` category
   - **Impact**: All variables default to `#9077c6` (may vary with LSP)

3. **Struct Field Visibility**:
   - GoLand: Different colors for public/private struct fields
   - Zed: Single `property` category
   - **Impact**: Both use `#9876aa`

4. **Interface vs Struct**:
   - GoLand: Different colors for interfaces and structs
   - Zed: Limited type differentiation
   - **Workaround**: Used `syntax.enum` for interface highlighting

## Rust Language Mappings

For Rust, the theme applies similar principles:

| Rust Element | Zed Category | Color | Source |
|--------------|--------------|-------|--------|
| Functions | `syntax.function` | `#ffe37b` | `GO_EXPORTED_FUNCTION` |
| Structs | `syntax.type` | `#c67f58` | `GO_PACKAGE_EXPORTED_STRUCT` |
| Enums | `syntax.enum` | `#c6a469` | `GO_PACKAGE_EXPORTED_INTERFACE` |
| Traits | `syntax.variant` | `#c6a469` | `GO_PACKAGE_EXPORTED_INTERFACE` |
| Macros | `syntax.attribute` | `#bbb529` | Annotation color |
| Constants | `syntax.constant` | `#9876aa` | `GO_LOCAL_CONSTANT` |
| Variables | `syntax.variable` | `#9077c6` | `GO_IDENTIFIER` |
| Self/self | `syntax.variable.special` | `#ffff00` | `GO_METHOD_RECEIVER` |

## UI Color Mappings

| UI Element | Color | Source |
|------------|-------|--------|
| Background | `#2b2d30` | Darcula background |
| Editor Background | `#1e1f22` | Darcula editor background |
| Editor Foreground | `#e8e8e8` | Darcula text (brighter) |
| Editor Active Line | `#25262a` | Darcula active line |
| Editor Gutter | `#1e1f22` | Same as editor background |
| Selection | `#214283` | Darcula selection |
| Tab Bar Background | `#22232a` | Darcula darker gray |
| Tab Inactive | `#22232a` | Same as tab bar (blends in) |
| Tab Active | `#3a3c41` | Medium gray (distinct from inactive) |
| Panel Background | `#22232a` | Darcula tool window |
| Status Bar | `#22232a` | Darcula status bar |
| Title Bar | `#22232a` | Darcula title bar |
| Toolbar | `#22232a` | Darcula toolbar |
| Border | `#2b2d30` | Darcula border |
| Border Variant | `#3a3c41` | Darcula border variant |
| Border Focus | `#3e6185` | Darcula focus color |
| Border Selected | `#3e6185` | Darcula selected border |
| Line Numbers | `#606366` | Darcula line numbers |
| Active Line Number | `#e8e8e8` | Bright text for active line |
| Element Hover | `#3a3c41` | Medium gray for hover state |
| Element Active | `#4b6eaf` | Blue for active elements |
| Element Selected | `#214283` | Dark blue for selection |
| Scrollbar Thumb | `#3a3c4180` | Semi-transparent gray |
| Scrollbar Hover | `#4e5254cc` | Lighter on hover |

## Color Preservation

The following colors have been adapted from the original scheme:

### Syntax Colors (Adapted)
- Functions: `#ffe66d` (yellow, close to original `#ffe37b`)
- Methods: `#61afef` (blue)
- Strings: `#50fa7b` (bright green, adapted from `#30be47`)
- Comments: `#7f848e` (gray, adapted from `#a1a1a1`)
- Constants: `#e4854e` (orange, adapted from `#9876aa`)
- Keywords: `#d689ff` (purple)
- Types: `#50fa7b` (green)
- Variables: `#ffffff` (white for better readability)
- Variable Parameters: `#d1556c` (red)
- Operators: `#83a598` (blue-gray)

### UI Colors (Schema Compliant)
- All UI colors follow Zed's official schema v0.1.0
- Tab distinction: Active tab (`#3a3c41`) vs Inactive (`#22232a`)
- Scrollbar uses underscore notation: `scrollbar_thumb.background`
- Proper border and element state colors defined

## Schema Compliance Notes

This theme has been validated against Zed's official theme schema v0.1.0:

1. **Tab Styling**
   - Uses only supported properties: `tab_bar.background`, `tab.active_background`, `tab.inactive_background`
   - Does NOT use `tab.active_border` (not in schema)
   - Active tab distinguished by brighter background color

2. **Scrollbar Naming**
   - Main thumb background uses underscore: `scrollbar_thumb.background`
   - Other scrollbar properties use dot notation as per schema

3. **All Properties Validated**
   - Every color property in the theme exists in the official schema
   - No custom or unsupported properties used

## Future Improvements

As Zed's semantic highlighting capabilities evolve, the theme can be enhanced to support:

1. **Exported vs Local Function Distinction**
   - Currently: Both use `#ffe66d`
   - Goal: Exported `#ffe66d`, Local `#ff7b00` (if supported)

2. **Variable Scope Differentiation**
   - Currently: Variables use `#ffffff`
   - Goal: Local `#6598c6`, Package `#816093`, Shadowing `#2ce7ac` (if LSP provides)

3. **Struct Field Visibility**
   - Currently: Properties use `#dfa180`
   - Goal: Public `#5bc600`, Private default (if semantic tokens support)

4. **Type Category Refinement**
   - Better distinction between structs, interfaces, enums
   - Utilize `#8996a5`, `#7d91c6` for private types (if semantic support)

5. **Tab Border Enhancement**
   - If Zed adds `tab.active_border` to schema, use `#4b6eaf` for stronger distinction

## Contributing Color Mappings

If you find better ways to map these colors in Zed:

1. Test with real code examples
2. Verify color contrast and readability
3. Document the mapping rationale
4. Submit a pull request with examples

## References

- Original scheme: `Colorful_Darcula.icls`
- Zed theme schema: https://zed.dev/schema/themes/v0.1.0.json
- JetBrains Darcula: Base color scheme