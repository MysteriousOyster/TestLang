## WARNING: Don't run this with big numbers, TestLang isn't TCO.
def math.mult num_multiplied times_multiplied
| var positive
| = positive
| gt times_multiplied 0
| var negative
| = negative
| !! positive
| math.mult_accumulator num_multiplied times_multiplied 0 positive negative
fed

def math.mult_accumulator num_multiplied_a times_multiplied_a accumulated positive_a negative_a
| var done
| = done
| eq times_multiplied_a 0
| var notdone
| = notdone
| !! done

| bif done
| > ech accumulated
| eif

| bif notdone
| > = accumulated
| > add accumulated num_multiplied_a
| > bif positive_a
| > > = times_multiplied_a
| > > add times_multiplied_a -1
| > eif
| > bif negative_a
| > > = times_multiplied_a
| > > add times_multiplied_a 1
| > eif
| > math.mult_accumulator num_multiplied_a times_multiplied_a accumulated
| eif
fed