# Browser Engine Development Changelog

## [0.1.6] - 2024-XX-XX

### Added
- Precise CSS width calculation algorithm for block layout
- Comprehensive handling of auto margins and width constraints
- Detailed width computation following CSS specification
- `lookup()` method for `StyledNode` to support fallback property resolution
- `to_px()` method for `Value` enum to convert values to pixels

### Changed
- Enhanced block width calculation method
- More robust handling of different width and margin scenarios
- Improved dimension computation logic
- Refactored layout methods to use references and resolve ownership issues
- Simplified property lookup and value conversion

### Fixed
- Resolved compilation errors related to method availability
- Addressed ownership and borrowing issues in layout module

## [0.1.5] - 2024-XX-XX

### Added
- Comprehensive block layout implementation
- Methods for calculating box dimensions and positioning
- Support for width, margin, padding, and border calculations
- Enhanced layout tree traversal with top-down and bottom-up passes

### Changed
- Refined block layout algorithm based on Matt Brubeck's tutorial
- Improved layout box dimension calculations
- Added methods for expanding rectangles and calculating box areas

## [0.1.4] - 2024-XX-XX

### Added
- Enhanced layout tree construction with mixed display type handling
- Support for anonymous block and inline containers
- Improved display property detection and processing

### Changed
- Refined layout tree generation algorithm
- Removed unused inline container method
- Improved test coverage for layout module

## [0.1.3] - 2024-XX-XX

### Added
- Layout module for creating layout trees
- Implemented `LayoutBox` and `Dimensions` structs
- Support for block and inline layout
- Anonymous block box generation
- Comprehensive unit tests for layout tree construction

### Changed
- Updated library module exports to include layout functionality

## [0.1.2] - 2024-XX-XX

### Added
- Style module for CSS selector matching and style tree construction
- Implemented `StyledNode` struct for managing styled DOM nodes
- Added selector matching for tag names, classes, and IDs
- Comprehensive unit tests for style module

### Fixed
- Resolved compilation errors in CSS and style modules
- Added `Clone` and `PartialEq` derives for `Unit` and `Color` types
- Corrected field name references in `ElementData`

### Changed
- Refactored CSS and style module type implementations
- Improved error handling and type compatibility

## [0.1.1] - 2024-01-08

### Fixed
- Resolved ambiguous `parse` function re-exports in `lib.rs`
- Removed unused imports in CSS parser
- Removed unused `starts_with` method in CSS parser
- Fixed irrefutable pattern matching in CSS tests
- Cleaned up unused code warnings
- Code cleanup
- Warning resolution

## [0.1.0] - 2024-01-07

### Added
- Initial project setup
- Basic DOM implementation
  - Node structure with support for Element and Text nodes
  - ElementData structure for tag names and attributes
  - Pretty printing functionality for DOM nodes
  - Unit tests for DOM operations

- HTML Parser implementation
  - Parser structure for tracking position in input
  - Support for parsing HTML elements, attributes, and text nodes
  - Automatic wrapping of text in HTML elements
  - Comprehensive test suite for HTML parsing
  - Support features:
    - Balanced tags (e.g., `<p>...</p>`)
    - Attributes with quoted values (e.g., `id="main"`)
    - Text nodes
    - Nested elements

- CSS Parser implementation
  - Stylesheet, Rule, Selector, and Declaration data structures
  - Support for parsing:
    - Simple selectors (tag, class, ID)
    - Specificity calculation
    - Basic CSS values (keywords, lengths, colors)
    - Declarations with property-value pairs
  - Comprehensive test suite for CSS parsing
  - Support features:
    - Tag selectors
    - Class selectors
    - ID selectors
    - Pixel length units
    - Hex color values

### Fixed
- HTML Parser improvements
  - Modified parse function to properly handle text-only content
  - Added proper wrapping of text nodes in HTML elements
  - Fixed test case for text node parsing
  - Improved type checking for element vs text nodes

### Technical Details
- Created core modules:
  - `dom.rs`: DOM data structures and operations
  - `html.rs`: HTML parsing functionality
  - `css.rs`: CSS parsing functionality
  - `lib.rs`: Module exports and public interface

### Testing
- Implemented test cases for:
  - DOM node creation and manipulation
  - Text node handling
  - Element node creation with attributes
  - Pretty printing of DOM trees
  - HTML parsing of various structures
  - Nested HTML elements
  - HTML attributes parsing
  - CSS selector parsing
  - CSS specificity calculation
  - CSS declaration parsing

### Not Yet Implemented
- Rendering engine
- Error handling for malformed HTML/CSS
- Support for:
  - Advanced CSS selectors
  - More CSS value types
  - Media queries
  - CSS inheritance
  - Complex CSS rules

### Next Steps
- Implement rendering engine
- Add more comprehensive error handling
- Expand CSS parsing capabilities
