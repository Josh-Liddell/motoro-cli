using JuliaHub

println("Authenticating")
JuliaHub.authenticate("https://juliahub.com")

println("Job submitted to JuliaHub!")
JuliaHub.submit_job(
    JuliaHub.script"""
    answer = 42 * 2
    println("Hello World")
    """noenv
)
