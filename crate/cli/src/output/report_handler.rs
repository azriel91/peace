use std::fmt::{self, Write};

use miette::{Diagnostic, GraphicalReportHandler, JSONReportHandler};

/// Bridging enum to allow delegation to
/// `miette::GraphicalReportHandler::render_report` and
/// `miette::JSONReportHandler::render_report`.
#[derive(Debug)]
pub(crate) enum ReportHandler {
    GraphicalReportHandler(GraphicalReportHandler),
    JsonReportHandler(JSONReportHandler),
    None,
}

impl ReportHandler {
    pub fn render_report(&self, f: &mut impl Write, diagnostic: &dyn Diagnostic) -> fmt::Result {
        match self {
            ReportHandler::GraphicalReportHandler(graphical_report_handler) => {
                graphical_report_handler.render_report(f, diagnostic)
            }
            ReportHandler::JsonReportHandler(json_report_handler) => {
                json_report_handler.render_report(f, diagnostic)
            }
            ReportHandler::None => Ok(()),
        }
    }
}

impl From<GraphicalReportHandler> for ReportHandler {
    fn from(graphical_report_handler: GraphicalReportHandler) -> Self {
        Self::GraphicalReportHandler(graphical_report_handler)
    }
}

impl From<JSONReportHandler> for ReportHandler {
    fn from(json_report_handler: JSONReportHandler) -> Self {
        Self::JsonReportHandler(json_report_handler)
    }
}
