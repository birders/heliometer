++++++++++		counter := 10
[			repeat 10 times
  >+++++++		counter *: 1 7
  >++++++++++		counter *: 1 7 10
  >+++			counter *: 1 7 10 3
  >+			counter *: 1 7 10 3 1
  <<<<-			decrement counter
]			0 70 100 30 10
>++.                    70 add 2  = H
>+.                     100 add 1 = e
+++++++..               101 add 7 = ll
+++.                    108 add 3 = o
>++.                    30 add 2  = SPC
# 0 72 111 32
#          ^
<<+++++++++++++++.      72 add 15 = W
>.                      111       = o
+++.                    111 add 3 = r
------.                 114 sub 6 = l
--------.               108 sub 8 = d
>+.			32 add 1  = !
# 0 87 100 33
#          ^
>.			0         = NUL
