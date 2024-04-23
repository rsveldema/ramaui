
use crate::{
    button::Button, content_page::ContentPage, grid::{ColumnDefinition, Grid, GridColumnDefinitions, GridRowDefinitions, RowDefinition}, label::Label, text_block::TextBlock, unknown_ui_elt::Unknown, window::Window
};

pub trait Visitor {
    fn visit_button(&mut self, b: &Button);
    fn visit_window(&mut self, w: &Window);
    fn visit_label(&mut self, l: &Label);
    fn visit_text_block(&mut self, t: &TextBlock);
    fn visit_grid(&mut self, g: &Grid);
    fn visit_grid_cols(&mut self, g: &GridColumnDefinitions);
    fn visit_grid_row(&mut self, g: &GridRowDefinitions);
    fn visit_col_def(&mut self, g: &ColumnDefinition);
    fn visit_row_def(&mut self, g: &RowDefinition);
    fn visit_content_page(&mut self, g: &ContentPage);
    fn visit_unknown(&mut self, g: &Unknown);
}
