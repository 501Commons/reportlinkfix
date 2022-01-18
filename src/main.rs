use ::std::*;
use std::io::{stdin, stdout, Read, Write};

fn pause() {
    // Keeps the terminal window open
    let mut stdout = stdout();
    stdout.write(b"Press Enter to close...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn main() {
    let mut original_link = String::new();
    println!("Input the N-Central Report Manager link and press enter:");
    // Read the original link from the terminal
    io::stdin()
        .read_line(&mut original_link)
        .expect("Error reading input");
    // Remove any space at the beginning or the end of the link
    let trimmed_link = original_link.trim();
    // Check whether the link is valid and if it's URL encoded. Then generate the new link
    if trimmed_link.starts_with("javascript:javascript:ui.root.externalPopupWindowPostToReportManager('http://501C-REPORTMGR.npowerseattle.lan/ReportInterface/Pages/menu.aspx?DataSourceID=1', '") {
        // Not URL encoded original_link example javascript:javascript:ui.root.externalPopupWindowPostToReportManager('http://501C-REPORTMGR.npowerseattle.lan/ReportInterface/Pages/menu.aspx?DataSourceID=1', '0123456789');
        let link_trim_start = trimmed_link.trim_start_matches("javascript:javascript:ui.root.externalPopupWindowPostToReportManager('http://501C-REPORTMGR.npowerseattle.lan/ReportInterface/Pages/menu.aspx?DataSourceID=1', '");
        let new_link = link_trim_start.trim_end_matches("');");
        println!("\nUse this link to access the Report Manager. You must be connected to DirectAccess or VPN:\nhttp://501c-reportmgr.npowerseattle.lan/ReportInterface/Pages/menu.aspx?SessionID={}&DataSourceID=1", new_link);
    } else if trimmed_link.starts_with("javascript:javascript:ui.root.externalPopupWindowPostToReportManager('http://501C-REPORTMGR.npowerseattle.lan/ReportInterface/Pages/menu.aspx?DataSourceID=1%27,%20%27") {
        // URL encoded original_link example javascript:javascript:ui.root.externalPopupWindowPostToReportManager('http://501C-REPORTMGR.npowerseattle.lan/ReportInterface/Pages/menu.aspx?DataSourceID=1%27,%20%270123456789%27);
        let link_trim_start = trimmed_link.trim_start_matches("javascript:javascript:ui.root.externalPopupWindowPostToReportManager('http://501C-REPORTMGR.npowerseattle.lan/ReportInterface/Pages/menu.aspx?DataSourceID=1%27,%20%27");
        let new_link = link_trim_start.trim_end_matches("%27);");
        println!("\nUse this link to access the Report Manager. You must be connected to DirectAccess or VPN:\nhttp://501c-reportmgr.npowerseattle.lan/ReportInterface/Pages/menu.aspx?SessionID={}&DataSourceID=1", new_link);
    } else {
        println!("Not a valid link");
    }
    // Keep the terminal window open
    pause();
}
