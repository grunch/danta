use crate::models::Attendee;
use simple_excel_writer::*;

pub fn generate_file(attendees: &Vec<Attendee>) -> String {
    let file = "./files/attendees.xlsx";
    let mut wb = Workbook::create(file);
    let mut sheet = wb.create_sheet("Asistentes Lightning Hackday");

    // set column width
    sheet.add_column(Column { width: 10.0 });
    sheet.add_column(Column { width: 30.0 });
    sheet.add_column(Column { width: 30.0 });
    sheet.add_column(Column { width: 60.0 });
    sheet.add_column(Column { width: 30.0 });

    wb.write_sheet(&mut sheet, |sheet_writer| {
        let sw = sheet_writer;
        sw.append_row(row!["Id", "Nombre", "Apellido", "email", "Pagó"])?;

        for (index, val) in attendees.iter().enumerate() {
            sw.append_row(row![
                &*val.id.to_string(),
                &*val.firstname,
                &*val.lastname,
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
