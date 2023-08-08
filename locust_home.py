from jtl_listener_service import JtlListener
from email import header
from locust import HttpUser, TaskSet, SequentialTaskSet, task,  between, events
from uuid import uuid4 as u4
from random import choice
import json


class UserBehavior(SequentialTaskSet):
    @task(1)
    def test(self):
        self.client.get('/')


class WebsiteUser(HttpUser):
    tasks = [UserBehavior]
    wait_time = between(0, 0)

@events.init.add_listener
def on_locust_init(environment, **_kwargs):
    JtlListener(env=environment, project_name="parallel", scenario_name="async_db", backend_url="http://be", listener_url="http://listener")

