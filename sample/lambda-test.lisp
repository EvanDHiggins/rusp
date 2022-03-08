(if (< 4 5) (write "Four less than five!\n") (write "Four greater than five!\n"))
(if (< 234 433) (write "234 less than 433!\n") (write "433 less than 234?\n"))
(defun writeln (x) (write x) (write "\n"))
(writeln "another-one")
(writeln ((lambda (x) x) "Lambda works!"))
(let
  writelambda
  (lambda (y) (writeln y))
  (writelambda "Wrote with a lambda!"))

(defun foo (y) (write y))

(foo "Wrote through a define!")

(defun countdown (x) (if (< x 1) (writeln "done!") (countdown (- x 1))))
(countdown 10)

(defun multiplestatements (x) (writeln x) (writeln "stuff!") (writeln x))
(multiplestatements "Hello, world")

((lambda (x) (writeln x)) 12)
