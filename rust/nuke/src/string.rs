//! Basic string buffer which is only used in context with the text editor
//! to manage and manipulate dynamic or fixed size string content. This is _NOT_
//! the default string handling method. The only instance you should have any contact
//! with this API is if you interact with an `nk_text_edit` object inside one of the
//! copy and paste functions and even there only for more advanced cases.