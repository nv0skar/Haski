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

    fn getMaxValueOfData(array: &Vec<f64>) -> f64 {
        let mut biggestValue: f64 = 0.0;
        for number in array {
            if biggestValue < number.clone() {
                biggestValue = number.clone()
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
        balanceHistory: &Vec<(usize, f64)>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new("report.png", (2294, 1490)).into_drawing_area();
        root.fill(&RGBColor(15, 18, 25))?;

        let dataMaxValue = getMaxValueOfData(&data.iter().map(|tick| tick.high).collect());

        let mut chart = ChartBuilder::on(&root)
            .margin(6)
            .x_label_area_size(60)
            .y_label_area_size(60)
            .caption(
                "Backtesting results",
                ("sans-serif", 32).into_font().with_color(&WHITE),
            )
            .build_cartesian_2d(
                fromTimestamp2Date(data[0].timestamp)
                    ..fromTimestamp2Date(data.last().unwrap().timestamp),
                0f64..dataMaxValue,
            )?;

        chart
            .configure_mesh()
            .light_line_style(&RGBColor(20, 23, 30))
            .draw()?;

        chart
            .draw_series(data.iter().map(|tick| {
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
            }))?
            .label("Value")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &WHITE));

        let maxValue2MaxBalanceRatio = dataMaxValue
            / getMaxValueOfData(&balanceHistory.iter().map(|balance| balance.1).collect());

        chart
            .draw_series(
                AreaSeries::new(
                    balanceHistory.iter().map(|balance| {
                        (
                            fromTimestamp2Date(data[balance.0].timestamp),
                            (balance.1 * maxValue2MaxBalanceRatio),
                        )
                    }),
                    0.0,
                    &BLUE.mix(0.2),
                )
                .border_style(&BLUE),
            )?
            .label("Balance (Scaled to fit)")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

        chart
            .draw_series(orders.iter().map(|order| {
                Circle::new(
                    (fromTimestamp2Date(data[order.0].timestamp), order.1 as f64),
                    10f32,
                    {
                        // println!("{:?}", order);
                        if order.2 == 0u8 {
                            &GREEN
                        } else if order.2 == 1u8 {
                            &RED
                        } else {
                            &MAGENTA
                        }
                    },
                )
            }))?
            .label("Orders")
            .legend(|(x, y)| Circle::new((x + 10, y), 4i32, &WHITE));

        chart
            .configure_series_labels()
            .label_font(("sans-serif", 32).into_font().with_color(&WHITE))
            .border_style(&WHITE)
            .background_style(&BLACK)
            .draw()?;

        // To avoid the IO failure being ignored silently, we manually call the present function
        root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
        println!("Result has been saved to {}", "report.png");

        Ok(())
    }
}
