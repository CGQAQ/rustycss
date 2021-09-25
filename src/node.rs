use crate::datatype::Number;
use std::collections::HashMap;

pub(crate) enum ChildNode {
    AtRule,
    Rule,
    Declaration,
    Comment,
}

pub(crate) enum AnyNode {
    AtRule,
    Rule,
    Declaration,
    Comment,
    Root,
    Document,
}

pub(crate) enum ChildProps {
    AtRuleProps,
    RuleProps,
    DeclarationProps,
    CommentProps,
}

pub(crate) enum NodeType {
    Root,
    AtRule,
    Rule,
    Decl,
    Comment,
}

pub(crate) enum ParentType {
    Document,
    Container,
    None,
}

#[allow(dead_code)]
pub(crate) struct Position {
    /// Source offset in file. It starts from 0.
    offset: Number,

    /// Source line in file. In contrast to `offset` it starts from 1.
    column: Number,

    /// Source column in file.
    line: Number,
}

pub(crate) struct Source {
    /// The file source of the node.
    input: i32, // TODO(CG): implement Input

    /// The starting position of the node’s source.
    start: Option<Position>,

    /// The ending position of the node's source.
    end: Option<Position>,
}

pub(crate) struct NodeProps {
    source: Option<Source>,
}

pub(crate) struct NodeErrorOptions {
    /// Plugin name that created this error. PostCSS will set it automatically.
    plugin: Option<String>,

    /// A word inside a node's string, that should be highlighted as source  of error.
    word: Option<String>,

    /// An index inside a node's string that should be highlighted as source of error.
    index: Option<Number>,
}

///
/// All node structs *should* implement the following trait
///
/// You should not extend this classes to create AST for selector or value parser.
///
/// Ref: https://github.com/postcss/postcss/blob/f20af5f909639416f8634e9c4d2f078b0e313071/lib/node.d.ts#L83
///
/// TODO(CG): Better doc
///
pub(crate) trait NodeTrait {
    ///
    /// string representing the node’s type. Possible values are `root`, `atrule`,
    /// `rule`, `decl`, or `comment`.
    /// ```js
    /// new Declaration({ prop: 'color', value: 'black' }).type //=> 'decl'
    /// ```
    ///
    fn get_type(&self) -> NodeType;

    ///
    ///  The node’s parent node.
    ///
    /// ```js
    ///  root.nodes[0].parent === root
    ///  ```
    fn get_parent(&self) -> ParentType;

    ///
    /// The input source of the node.
    ///
    /// The property is used in source map generation.
    /// If you create a node manually (e.g., with `postcss.decl()`),
    /// that node will not have a `source` property and will be absent
    /// from the source map. For this reason, the plugin developer should
    /// consider cloning nodes to create new ones (in which case the new node’s
    /// source will reference the original, cloned node) or setting
    /// the `source` property manually.
    /// ```js
    /// decl.source.input.from //=> '/home/ai/a.sass'
    /// decl.source.start      //=> { line: 10, column: 2 }
    /// decl.source.end        //=> { line: 10, column: 12 }
    /// ```
    /// ```js
    /// // Bad
    /// const prefixed = postcss.decl({
    ///     prop: '-moz-' + decl.prop,
    ///     value: decl.value
    /// })
    /// // Good
    /// const prefixed = decl.clone({ prop: '-moz-' + decl.prop })
    /// ```
    /// ```js
    /// if (atrule.name === 'add-link') {
    /// const rule = postcss.rule({ selector: 'a', source: atrule.source })
    /// atrule.parent.insertBefore(atrule, rule)
    /// }
    /// ```
    ///
    fn get_source(&self) -> Option<Source>;

    ///
    /// Information to generate byte-to-byte equal node string as it was
    /// in the origin input.
    /// Every parser saves its own properties,
    /// but the default CSS parser uses:
    ///     * `before`: the space symbols before the node. It also stores `*`
    ///       and `_` symbols before the declaration (IE hack).
    ///     * `after`: the space symbols after the last child of the node
    ///       to the end of the node.
    ///     * `between`: the symbols between the property and value
    ///       for declarations, selector and `{` for rules, or last parameter
    ///       and `{` for at-rules.
    ///     * `semicolon`: contains true if the last child has
    ///       an (optional) semicolon.
    ///     * `afterName`: the space between the at-rule name and its parameters.
    ///     * `left`: the space symbols between `/*` and the comment’s text.
    ///     * `right`: the space symbols between the comment’s text
    ///       and <code>*&#47;</code>.
    ///     * `important`: the content of the important statement,
    ///       if it is not just `!important`.
    ///
    /// PostCSS cleans selectors, declaration values and at-rule parameters
    /// from comments and extra spaces, but it stores origin content in raws
    /// properties. As such, if you don’t change a declaration’s value,
    /// PostCSS will use the raw value with comments.
    ///
    /// ```js
    /// const root = postcss.parse('a {\n  color:black\n}')
    /// root.first.first.raws //=> { before: '\n  ', between: ':' }
    /// ```
    ///
    fn get_raws(&self) -> HashMap<String, String>;
}

#[test]
fn test_hello_world() {}
