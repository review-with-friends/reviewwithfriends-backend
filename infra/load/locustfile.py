from locust import HttpUser, task 

# Instantiate a new virtual user
class HelloWorldUser(HttpUser): 
    # This tells locust to treat the method below 
    # as something the virtual user would do
    @task 
    # Define a new method
    def hello_world(self): 
        self.client.get("/ping") 