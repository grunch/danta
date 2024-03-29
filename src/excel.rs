use crate::models::Attendee;
use simple_excel_writer::*;

pub fn generate_file(attendees: &[Attendee]) -> String {
    let file = "./files/attendees.xlsx";
    let mut wb = Workbook::create(file);
    let mut sheet = wb.create_sheet("Asistentes Lightning Hackday");

    // set column width
    sheet.add_column(Column { width: 10.0 });
    sheet.add_column(Column { width: 30.0 });
    sheet.add_column(Column { width: 30.0 });
    sheet.add_column(Column { width: 40.0 });
    sheet.add_column(Column { width: 30.0 });

    wb.write_sheet(&mut sheet, |sheet_writer| {
        let sw = sheet_writer;
        sw.append_row(row!["Id", "Nombre", "github", "E-mail", "Pagó"])?;

        for val in attendees.iter() {
            sw.append_row(row![
                &*val.id.to_string(),
                &*val.firstname,
                &*val.data1,
                &*val.email,
                &*val.paid.to_string()
            ])?;
        }

        sw.append_row(row![blank!(4)])
    })
    .expect("write excel error!");

    wb.close().expect("close excel error!");

    file.to_string()
}
