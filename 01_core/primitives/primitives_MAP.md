# 🗺️ Module: primitives

| File | Purpose |
|---|---|
| `Cargo.toml` | — |
| `src/diag.rs` | Diagnostics. |
| `src/engine.rs` | Definition of the central compilation context. |
| `src/foundations/args.rs` | — |
| `src/foundations/array.rs` | — |
| `src/foundations/auto.rs` | — |
| `src/foundations/bool.rs` | A type with two states. |
| `src/foundations/bytes.rs` | — |
| `src/foundations/calc.rs` | Calculations and processing of numeric values. |
| `src/foundations/cast.rs` | — |
| `src/foundations/content/element.rs` | — |
| `src/foundations/content/field.rs` | — |
| `src/foundations/content/mod.rs` | — |
| `src/foundations/content/packed.rs` | — |
| `src/foundations/content/raw.rs` | — |
| `src/foundations/content/vtable.rs` | A custom [vtable] implementation for content. |
| `src/foundations/context.rs` | Data that is contextually made available to code. |
| `src/foundations/datetime.rs` | — |
| `src/foundations/decimal.rs` | — |
| `src/foundations/dict.rs` | — |
| `src/foundations/duration.rs` | Represents a positive or negative span of time. |
| `src/foundations/fields.rs` | Fields on values. |
| `src/foundations/float.rs` | — |
| `src/foundations/func.rs` | — |
| `src/foundations/int.rs` | — |
| `src/foundations/label.rs` | A label for an element. |
| `src/foundations/mod.rs` | Foundational types and functions. |
| `src/foundations/module.rs` | A collection of variables and functions that are commonly related to |
| `src/foundations/none.rs` | — |
| `src/foundations/ops.rs` | Operations on values. |
| `src/foundations/plugin.rs` | — |
| `src/foundations/repr.rs` | Debug representation of values. |
| `src/foundations/scope.rs` | — |
| `src/foundations/selector.rs` | — |
| `src/foundations/str.rs` | — |
| `src/foundations/styles.rs` | — |
| `src/foundations/symbol.rs` | — |
| `src/foundations/sys.rs` | System-related things. |
| `src/foundations/target.rs` | The export target. |
| `src/foundations/ty.rs` | — |
| `src/foundations/value.rs` | — |
| `src/foundations/version.rs` | — |
| `src/introspection/convergence.rs` | — |
| `src/introspection/counter.rs` | — |
| `src/introspection/here.rs` | Provides the current location in the document. |
| `src/introspection/introspector.rs` | — |
| `src/introspection/locate.rs` | Determines the location of an element in the document. |
| `src/introspection/location.rs` | — |
| `src/introspection/locator.rs` | — |
| `src/introspection/metadata.rs` | Exposes a value to the query system without producing visible content. |
| `src/introspection/mod.rs` | Interaction between document parts. |
| `src/introspection/query.rs` | — |
| `src/introspection/state.rs` | — |
| `src/introspection/tag.rs` | Marks the start or end of a locatable element. |
| `src/layout/abs.rs` | An absolute length. |
| `src/layout/align.rs` | — |
| `src/layout/angle.rs` | — |
| `src/layout/axes.rs` | — |
| `src/layout/columns.rs` | Separates a region into multiple equally sized columns. |
| `src/layout/container.rs` | — |
| `src/layout/corners.rs` | — |
| `src/layout/dir.rs` | The four directions into which content can be laid out. |
| `src/layout/em.rs` | — |
| `src/layout/fr.rs` | — |
| `src/layout/fragment.rs` | A partial layout result. |
| `src/layout/frame.rs` | Finished documents. |
| `src/layout/grid/mod.rs` | — |
| `src/layout/grid/resolve.rs` | — |
| `src/layout/hide.rs` | Hides content without affecting layout. |
| `src/layout/layout.rs` | Provides access to the current outer container's (or page's, if none) |
| `src/layout/length.rs` | — |
| `src/layout/measure.rs` | — |
| `src/layout/mod.rs` | Composable layouts. |
| `src/layout/pad.rs` | Adds spacing around content. |
| `src/layout/page.rs` | — |
| `src/layout/place.rs` | Places content relatively to its parent container. |
| `src/layout/point.rs` | A point in 2D. |
| `src/layout/ratio.rs` | A ratio of a whole. |
| `src/layout/rect.rs` | A rectangle in 2D. |
| `src/layout/regions.rs` | A single region to layout into. |
| `src/layout/rel.rs` | — |
| `src/layout/repeat.rs` | Repeats content to the available space. |
| `src/layout/sides.rs` | — |
| `src/layout/size.rs` | A size in 2D. |
| `src/layout/spacing.rs` | Inserts horizontal spacing into a paragraph. |
| `src/layout/stack.rs` | Arranges content and spacing horizontally or vertically. |
| `src/layout/transform.rs` | Moves content without affecting layout. |
| `src/lib.rs` | Typst's standard library. |
| `src/loading/cbor.rs` | Reads structured data from a CBOR file. |
| `src/loading/csv.rs` | Reads structured data from a CSV file. |
| `src/loading/json.rs` | Reads structured data from a JSON file. |
| `src/loading/mod.rs` | Data loading. |
| `src/loading/read.rs` | Reads plain text or data from a file. |
| `src/loading/toml.rs` | Reads structured data from a TOML file. |
| `src/loading/xml.rs` | Reads structured data from an XML file. |
| `src/loading/yaml.rs` | Reads structured data from a YAML file. |
| `src/math/accent.rs` | — |
| `src/math/attach.rs` | A base with optional attachments. |
| `src/math/cancel.rs` | Displays a diagonal line over a part of an equation. |
| `src/math/equation.rs` | — |
| `src/math/frac.rs` | A mathematical fraction. |
| `src/math/lr.rs` | — |
| `src/math/matrix.rs` | — |
| `src/math/mod.rs` | Mathematical formulas. |
| `src/math/op.rs` | A text operator in an equation. |
| `src/math/root.rs` | A square root. |
| `src/math/style.rs` | Bold font style in math. |
| `src/math/underover.rs` | A horizontal line under content. |
| `src/model/bibliography.rs` | — |
| `src/model/cite.rs` | — |
| `src/model/document.rs` | — |
| `src/model/emph.rs` | Emphasizes content by toggling italics. |
| `src/model/enum.rs` | — |
| `src/model/figure.rs` | — |
| `src/model/footnote.rs` | — |
| `src/model/heading.rs` | — |
| `src/model/link.rs` | — |
| `src/model/list.rs` | — |
| `src/model/mod.rs` | Structuring elements that define the document model. |
| `src/model/numbering.rs` | — |
| `src/model/outline.rs` | — |
| `src/model/par.rs` | — |
| `src/model/quote.rs` | — |
| `src/model/reference.rs` | — |
| `src/model/strong.rs` | Strongly emphasizes content by increasing the font weight. |
| `src/model/table.rs` | — |
| `src/model/terms.rs` | A list of terms and their descriptions. |
| `src/model/title.rs` | A document title. |
| `src/pdf/accessibility.rs` | — |
| `src/pdf/attach.rs` | A file that will be attached to the output PDF. |
| `src/pdf/mod.rs` | PDF-specific functionality. |
| `src/routines.rs` | — |
| `src/symbols.rs` | Modifiable symbols. |
| `src/text/case.rs` | Converts a string or content to lowercase. |
| `src/text/deco.rs` | Underlines text. |
| `src/text/font/book.rs` | — |
| `src/text/font/color.rs` | Utilities for color font handling |
| `src/text/font/exceptions.rs` | — |
| `src/text/font/mod.rs` | Font handling. |
| `src/text/font/variant.rs` | Properties that distinguish a font from other fonts in the same family. |
| `src/text/item.rs` | — |
| `src/text/lang.rs` | — |
| `src/text/linebreak.rs` | Inserts a line break. |
| `src/text/lorem.rs` | Creates blind text. |
| `src/text/mod.rs` | Text handling. |
| `src/text/raw.rs` | — |
| `src/text/shift.rs` | Renders text in subscript. |
| `src/text/smallcaps.rs` | Displays text in small capitals. |
| `src/text/smartquote.rs` | — |
| `src/text/space.rs` | A text space. |
| `src/visualize/color.rs` | — |
| `src/visualize/curve.rs` | — |
| `src/visualize/gradient.rs` | — |
| `src/visualize/image/mod.rs` | Image handling. |
| `src/visualize/image/pdf.rs` | A PDF document. |
| `src/visualize/image/raster.rs` | — |
| `src/visualize/image/svg.rs` | — |
| `src/visualize/line.rs` | A line from one point to another. |
| `src/visualize/mod.rs` | Drawing and visualization. |
| `src/visualize/paint.rs` | How a fill or stroke should be painted. |
| `src/visualize/path.rs` | A path through a list of points, connected by Bézier curves. |
| `src/visualize/polygon.rs` | A closed polygon. |
| `src/visualize/shape.rs` | A rectangle with optional content. |
| `src/visualize/stroke.rs` | — |
| `src/visualize/tiling.rs` | — |
