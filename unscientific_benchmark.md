# UNSCIENTIFIC EXPERIMENT CONDITIONS

* input file: `example_s.png`
* outer iterations: `10`
* inner iterations: `100_000`
* max line length: `50`
* output file: `test.png`
* no output in inner iteration

## results

* `~12000ns/iter`
* `~1200ns/iter`
    + after changing line generation function
    + previously, would keep generating arbitrary lines until they were <= max
    + now choose a point, an angle, and a magnitude and create a line like that
