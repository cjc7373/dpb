from django.db import models


class Pastebin(models.Model):
    key = models.TextField(db_index=True, primary_key=True)
    content = models.TextField()
    language = models.TextField(blank=True)
    created_time = models.DateTimeField(auto_now_add=True, db_index=True)
    expire_time = models.DateTimeField(null=True, blank=True, db_index=True)
    length = models.IntegerField()

    def __str__(self):
        return self.key


class Misc(models.Model):
    key = models.TextField(db_index=True, primary_key=True)
    value = models.TextField()
