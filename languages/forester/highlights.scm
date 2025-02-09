(comment) @comment

[
 "\\"
 "("
 ")"
 "{"
 "}"
 "["
 "]"
] @punctuation.bracket

(p "p" @function.builtin)
(li "li" @function.builtin)
(ul "ul"  @function.builtin)
(ol "ol"  @function.builtin)
(em "em"  @function.builtin)
(strong "strong" @function.builtin)
(code "code" @function.builtin)

(tex "tex" @function.builtin)

(ident (text) @field)

(subtree "subtree" @keyword)

(transclude "transclude" @keyword)
(transclude address: (_) @string)

(def "def" @keyword)
(let "let" @keyword)
(object "object" @constant)
(object self: (_) @keyword)
(method_decl key: (_) @method)
(patch "patch" @text.diff.add)
(patch object: (_) @constant)

(markdown_link label: (_) @label)
(markdown_link dest: (_) @text.uri)
(unlabeled_link (external_link)) @text.uri

(scope "scope" @namespace)
(put "put" @variable.parameter)

(query_tree "query" @keyword)

(import "import" @keyword)
(export "export" @keyword)
(transclude "transclude" @keyword)

(em (text) @emphasis)
(strong (text) @emphasis.strong)
