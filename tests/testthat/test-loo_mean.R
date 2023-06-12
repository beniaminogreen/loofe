test_that("leave_one_out_mean works", {
  expect_equal(c(2.5,2.0,1.5), loo_mean(c(1,2,3)))

  expect_equal(
               c(3,3,1,1),
               loo_mean(
                        c(1,1,3,3),
                        c(1,1,2,2)
               )
  )
})
