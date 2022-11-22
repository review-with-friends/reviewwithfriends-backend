from locust import HttpUser, task 

# Instantiate a new virtual user
class HelloWorldUser(HttpUser): 
    # This tells locust to treat the method below 
    # as something the virtual user would do
    @task 
    # Define a new method
    def hello_world(self): 
        self.client.headers["Authorization"] = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJhdWQiOiJtb2IiLCJleHAiOjE2NzEyODk0MDQsInN1YiI6IjRlZDFhYzk5LTBkM2QtNDRiOC1iYTJhLTAzYWFkZTk5ZTM1MCJ9.rwE4Be63XBren-b5P5B11ETHQ5dz9GzGjL6SZHSRNLI"
        self.client.get("/api/v1/pic/profile_pic?user_id=4ed1ac99-0d3d-44b8-ba2a-03aade99e350") 