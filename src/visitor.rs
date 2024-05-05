
use crate::{
    button::Button, content_page::ContentPage, grid_layout::{ColumnDefinition, GridColumnDefinitions, GridLayout, GridRowDefinitions, RowDefinition}, label::Label, stack_layout::StackLayout, text_block::TextBlock, unknown_ui_elt::Unknown, window::Window
};

pub trait Visitor {
    fn start_visit_button(&mut self, b: & Button);
    fn start_visit_window(&mut self, w: & Window);
    fn start_visit_label(&mut self, l: & Label);
    fn start_visit_text_block(&mut self, t: & TextBlock);
    fn start_visit_grid(&mut self, g: & GridLayout);
    fn start_visit_grid_cols(&mut self, g: & GridColumnDefinitions);
    fn start_visit_grid_row(&mut self, g: & GridRowDefinitions);
    fn start_visit_col_def(&mut self, g: & ColumnDefinition);
    fn start_visit_row_def(&mut self, g: & RowDefinition);
    fn start_visit_content_page(&mut self, g: & ContentPage);
    fn start_visit_unknown(&mut self, g: & Unknown);
    fn start_visit_stack(&mut self, g: & StackLayout);

    
    fn visit_button(&mut self, b: & Button);
    fn visit_window(&mut self, w: & Window);
    fn visit_label(&mut self, l: & Label);
    fn visit_text_block(&mut self, t: & TextBlock);
    fn visit_grid(&mut self, g: & GridLayout);
    fn visit_grid_cols(&mut self, g: & GridColumnDefinitions);
    fn visit_grid_row(&mut self, g: & GridRowDefinitions);
    fn visit_col_def(&mut self, g: & ColumnDefinition);
    fn visit_row_def(&mut self, g: & RowDefinition);
    fn visit_content_page(&mut self, g: & ContentPage);
    fn visit_unknown(&mut self, g: & Unknown);
    fn visit_stack(&mut self, g: & StackLayout);
}
