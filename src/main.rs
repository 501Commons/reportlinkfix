use ::std::*;

fn main() {
    let mut original_link = String::new();
    println!("Input the N-Central Report Manager Link:");
    io::stdin()
        .read_line(&mut original_link)
        .expect("Error reading input");
    let trimmed_link = original_link.trim();
    if trimmed_link.starts_with("javascript:javascript:ui.root.externalPopupWindowPostToReportManager('http://501C-REPORTMGR.npowerseattle.lan/ReportInterface/Pages/menu.aspx?DataSourceID=1', '") {
        // Firefox original_link javascript:javascript:ui.root.externalPopupWindowPostToReportManager('http://501C-REPORTMGR.npowerseattle.lan/ReportInterface/Pages/menu.aspx?DataSourceID=1', '1917965022');
        let link_trim_start = trimmed_link.trim_start_matches("javascript:javascript:ui.root.externalPopupWindowPostToReportManager('http://501C-REPORTMGR.npowerseattle.lan/ReportInterface/Pages/menu.aspx?DataSourceID=1', '");
        let new_link = link_trim_start.trim_end_matches("');");
        println!("Firefox Link");
        println!("http://501c-reportmgr.npowerseattle.lan/ReportInterface/Pages/menu.aspx?SessionID={}&DataSourceID=1", new_link);
    } else if trimmed_link.starts_with("javascript:javascript:ui.root.externalPopupWindowPostToReportManager('http://501C-REPORTMGR.npowerseattle.lan/ReportInterface/Pages/menu.aspx?DataSourceID=1%27,%20%27") {
        // Chrome/Edge original_link javascript:javascript:ui.root.externalPopupWindowPostToReportManager('http://501C-REPORTMGR.npowerseattle.lan/ReportInterface/Pages/menu.aspx?DataSourceID=1%27,%20%272076719063%27);
        let link_trim_start = trimmed_link.trim_start_matches("javascript:javascript:ui.root.externalPopupWindowPostToReportManager('http://501C-REPORTMGR.npowerseattle.lan/ReportInterface/Pages/menu.aspx?DataSourceID=1%27,%20%27");
        let new_link = link_trim_start.trim_end_matches("%27);");
        println!("Chrome/Edge Link");
        println!("http://501c-reportmgr.npowerseattle.lan/ReportInterface/Pages/menu.aspx?SessionID={}&DataSourceID=1", new_link);
    } else {
        println!("Not a valid link");
    }
}
