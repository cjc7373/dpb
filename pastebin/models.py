from django.db import models


class Pastebin(models.Model):
    url = models.TextField()
    content = models.TextField()
