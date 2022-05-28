// Haski
// Copyright (C) 2022 ItsTheGuy
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

pub mod plot {
    use chrono::TimeZone;
    use plotters::prelude::*;
    use yahoo_finance_api::Quote;

    fn getMaxValueOfData(data: &Vec<Quote>) -> f64 {
        let mut biggestValue: f64 = 0.0;
        for tick in data {
            if biggestValue < tick.high {
                biggestValue = tick.high
            }
        }
        biggestValue
    }

    fn fromTimestamp2Date(timestamp: u64) -> crate::Date<crate::Local> {
        crate::Local.timestamp(timestamp as i64, 0).date()
    }

    pub fn draw(
        data: &Vec<Quote>,
        orders: &Vec<(usize, f64, u8)>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new("report.png", (2294, 1490)).into_drawing_area();
        root.fill(&RGBColor(15, 18, 25))?;

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(60)
            .y_label_area_size(60)
            .caption(
                "Backtesting results",
                ("sans-serif", 32).into_font().with_color(&WHITE),
            )
            .build_cartesian_2d(
                fromTimestamp2Date(data[0].timestamp)
                    ..fromTimestamp2Date(data.last().unwrap().timestamp),
                0f64..getMaxValueOfData(&data),
            )?;

        chart
            .configure_mesh()
            .light_line_style(&RGBColor(20, 23, 30))
            .draw()?;

        chart.draw_series(data.iter().map(|tick| {
            CandleStick::new(
                fromTimestamp2Date(tick.timestamp),
                tick.open,
                tick.high,
                tick.low,
                tick.close,
                &GREEN,
                &RED,
                2,
            )
        }))?;

        chart.draw_series(orders.iter().map(|order| {
            Circle::new(
                (fromTimestamp2Date(data[order.0].timestamp), order.1 as f64),
                10f32,
                {
                    if order.2 == 0 {
                        &GREEN
                    } else if order.2 == 1 {
                        &RED
                    } else {
                        &WHITE
                    }
                },
            )
        }))?;

        // To avoid the IO failure being ignored silently, we manually call the present function
        root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
        println!("Result has been saved to {}", "report.png");

        Ok(())
    }
}
