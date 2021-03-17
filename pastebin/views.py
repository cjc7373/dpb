from django.db import IntegrityError
from django.shortcuts import get_object_or_404, redirect, render
from django.utils.crypto import get_random_string
from django.views import View

from .forms import PastebinForm
from .models import Pastebin


class Index(View):
    def get(self, request):
        return render(request, 'index.html', {'form': PastebinForm()})

    def post(self, request):
        form = PastebinForm(request.POST)
        if form.is_valid():
            while True:
                try:
                    key = get_random_string(4)
                    Pastebin.objects.create(key=key, content=form.cleaned_data['content'])
                    return redirect(f'/{key}/')
                except IntegrityError:
                    pass
        else:
            return render(request, 'index.html', {'form': form})


class Snippet(View):
    def get(self, request, key):
        snippet = get_object_or_404(Pastebin, key=key)
        return render(request, 'code.html', {'code': snippet.content})
