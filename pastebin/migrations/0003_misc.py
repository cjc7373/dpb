# Generated by Django 4.0.2 on 2022-02-12 04:47

from django.db import migrations, models


class Migration(migrations.Migration):

    dependencies = [
        ('pastebin', '0002_auto_20210319_0433'),
    ]

    operations = [
        migrations.CreateModel(
            name='Misc',
            fields=[
                ('key', models.TextField(db_index=True, primary_key=True, serialize=False)),
                ('value', models.TextField()),
            ],
        ),
    ]
