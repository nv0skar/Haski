<h1 style="color:#99E7C8;text-shadow: -2px 2px #2E37D1;font-size:40px", align="center">ハスキー</h1>

<h4 align="center">Experimental hash-powered stock forecaster 👽</h4>

## <a name="what"></a>何？ ⭐️
ハスキー (also called **Haski**) is an statistical forecast algorithm conceived to be a stock forecaster.

## <a name="how"></a>どうやって？ 🚀
The Haski's algorithm iterate through all the values in a dataset. The process of finding a pattern for a value is:
1. Calculate the `deviation` which is defined by the following formula:
    ```
    deviation = |((f / n * 100) - 100|
    ```

    Where:
   - `f` average of a given number of values after the current one
   - `n` the current value
2. Check if `deviation` is greater than a given number, if it's not the following steps are skipped.
3. If `deviation` is greater than `0` an up trend is predicted, otherwise a down trend.
4. Then, fetch a number of values previous to the current value and for each each of those values calculate the `back2FrontDeviation` defined by the following formula (note that for the first value the `back2FrontDeviation` is not calculated):
    ```
    back2FrontDeviation = round(ln(|((h / z) * 100) - 100|))
    ```

    Where:
   - `h` one of those previous values
   - `z` the value before `h`
    And:
   - `round()` round the number to have no decimals
5. Lastly, calculate a hash of all of the `back2FrontDeviation` values obtained.

**I hope that I didn't forget anything 🥲**
## <a name="development"></a>発達 🧑‍💻
### <a name="developmentTODO"></a>リストを行う 🛸
- [x] Add balance simulation to backtesting