# UNSCIENTIFIC EXPERIMENT CONDITIONS

* input file: `example_s.png`
* outer iterations: `10`
* inner iterations: `100_000`
* fixed line length: `25`
* output file: `test.png`
* no output in inner iteration

## results

1. `~12000ns/iter`
2. `~5000ns/iter`
    * after changing line generation function
    * previously, would keep generating arbitrary lines until they were <= max
    * now choose a point, an angle, and a magnitude and create a line like that
    * let's also arbitrarily say that the average line length before was 25
        + it used to be anything <= 50 but i don't really know stats so...
3. `~1000ns/iter`
	* after adding the bresenham line algorithm
4. `~900ns/iter`
	* switching to manhattan distance instance of euclidean
5. `~875ns/iter`
	* make distance function return u32 instead of u64
