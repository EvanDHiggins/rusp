(if (< 4 5) (write "Four less than five!") (write "Four greater than five!"))
(if (< 234 433) (write "234 less than 433!") (write "433 less than 234?"))
(write "another-one")
(write ((lambda (x) x) "Lambda works!"))
(let
  writelambda
  (lambda (y) (write y))
  (writelambda "Wrote with a lambda!"))

(define foo (lambda (y) (write y)))

(foo "Wrote through a define!")

(define countdown (lambda (x) (if (< x 1) (write "done!") (countdown (- x 1)))))

(countdown 10)
