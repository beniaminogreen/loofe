#' Return string `"Hello world!"` to R.
#' @export
loo_mean <- function(x, cluster = NULL) {
   x <- as.numeric(x)
   if (is.null(cluster)){
        return(rust_loo_mean(x))
    } else{
        cluster = as.integer(as.factor(cluster))
        return(rust_loo_clustered_mean(x, cluster))
    }
}

