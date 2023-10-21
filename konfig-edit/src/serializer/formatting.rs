use std::fmt;

pub trait DocLineEscape {
    fn escape(&self, line: &mut String, gt_sign_pos: usize);
}

impl<F> DocLineEscape for F
where
    F: Fn(&mut String, usize),
{
    #[inline]
    fn escape(&self, line: &mut String, gt_sign_pos: usize) {
        (self)(line, gt_sign_pos)
    }
}

pub struct MarkdowDocLineEscape;

impl DocLineEscape for MarkdowDocLineEscape {
    #[inline]
    fn escape(&self, line: &mut String, gt_sign_pos: usize) {
        let rest = line.split_off(gt_sign_pos);

        line.push_str("<span>&gt;</span>");
        line.push_str(rest.split_at(1).1);
    }
}

pub struct FormattingOptions {
    pub doc_line_escape: Box<dyn DocLineEscape>,
    pub path_wrap_at_len: usize,
}

impl Default for FormattingOptions {
    #[inline]
    fn default() -> Self {
        Self {
            doc_line_escape: Box::new(MarkdowDocLineEscape),
            path_wrap_at_len: 100,
        }
    }
}

impl fmt::Debug for FormattingOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormattingOptions")
            .field("doc_line_escape", &"Box<dyn ...>")
            .field("path_wrap_at_len", &self.path_wrap_at_len)
            .finish()
    }
}
