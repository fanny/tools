initSidebarItems({"fn":[["block_indent","Inserts a hard line break before and after the content and increases the indention level for the content by one."],["comment","Marks some content as a comment trivia."],["dynamic_token","Create a token from a dynamic string and a range of the input source"],["empty_line","A forced empty line. An empty line inserts enough line breaks in the output for the previous and next element to be separated by an empty line."],["expand_parent","IR element that forces the parent group to print in expanded mode."],["format_once","Creates an inline `Format` object that can only be formatted once."],["format_with","Creates an object implementing `Format` that calls the passed closure to perform the formatting."],["get_lines_before","Get the number of line breaks between two consecutive SyntaxNodes in the tree"],["group_elements","Creates a logical `Group` around the content that should either consistently be printed on a single line or broken across multiple lines."],["hard_line_break","A forced line break that are always printed. A hard line break forces any enclosing `Group` to be printed over multiple lines."],["if_group_breaks","Adds a conditional content that is emitted only if it isn’t inside an enclosing `Group` that is printed on a single line. The element allows, for example, to insert a trailing comma after the last array element only if the array doesn’t fit on a single line."],["if_group_fits_on_line","Adds a conditional content specific for `Group`s that fit on a single line. The content isn’t emitted for `Group`s spanning multiple lines."],["indent","It adds a level of indentation to the given content"],["labelled","Marks some content with a label."],["line_suffix","Pushes some content to the end of the current line"],["line_suffix_boundary","Inserts a boundary for line suffixes that forces the printer to print all pending line suffixes. Helpful if a line sufix shouldn’t pass a certain point."],["soft_block_indent","Indents the content by inserting a line break before and after the content and increasing the indention level for the content by one if the enclosing group doesn’t fit on a single line. Doesn’t change the formatting if the enclosing group fits on a single line."],["soft_line_break","A line break that only gets printed if the enclosing `Group` doesn’t fit on a single line. It’s omitted if the enclosing `Group` fits on a single line. A soft line break is identical to a hard line break when not enclosed inside of a `Group`."],["soft_line_break_or_space","A line break if the enclosing `Group` doesn’t fit on a single line, a space otherwise."],["soft_line_indent_or_space","If the enclosing `Group` doesn’t fit on a single line, inserts a line break and indent. Otherwise, just inserts a space."],["space_token","Inserts a single space. Allows to separate different tokens."],["syntax_token_cow_slice","String that is the same as in the input source text if `text` is [`Cow::Borrowed`] or some replaced content if `text` is [`Cow::Owned`]."],["syntax_token_text_slice","Copies a source text 1:1 into the output text."],["token","Creates a token that gets written as is to the output. Make sure to properly escape the text if it’s user generated (e.g. a string and not a language keyword)."]],"struct":[["BestFitting","The first variant is the most flat, and the last is the most expanded variant. See [`best_fitting!`] macro for a more in-detail documentation"],["BlockIndent",""],["DynamicToken",""],["ExpandParent",""],["FillBuilder","Builder to fill as many elements as possible on a single line."],["FormatComment",""],["FormatLabelled",""],["FormatOnce",""],["FormatWith","Utility for formatting some content with an inline lambda function."],["GroupElements",""],["IfGroupBreaks",""],["Indent",""],["JoinBuilder","Builder to join together a sequence of content. See [Formatter::join]"],["JoinNodesBuilder","Builder to join together nodes that ensures that nodes separated by empty lines continue to be separated by empty lines in the formatted output."],["Line",""],["LineSuffix",""],["LineSuffixBoundary",""],["PrinterOptions","Options that affect how the [crate::Printer] prints the format tokens"],["Space",""],["StaticToken",""],["SyntaxTokenCowSlice",""],["SyntaxTokenTextSlice",""]],"trait":[["BufferExtensions",""],["MemoizeFormat","Utility trait that allows memorizing the output of a [Format]. Useful to avoid re-formatting the same object twice."],["_","Utility trait used to simplify the formatting of optional objects that are formattable."]]});