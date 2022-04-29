from django.db import IntegrityError
from django.http import HttpResponse
from django.shortcuts import get_object_or_404, redirect, render
from django.utils.crypto import get_random_string
from django.views import View
from pygments import highlight
from pygments.formatters import HtmlFormatter
from pygments.lexers import guess_lexer

from .forms import PastebinForm
from .models import Pastebin


def create_paste(content: str) -> Pastebin:
    while True:
        key_len = 4
        try:
            key = get_random_string(key_len)
            paste = Pastebin.objects.create(key=key, content=content, length=len(content))
            return paste
        except IntegrityError:
            key_len += 1


class Index(View):
    def get(self, request):
        return render(request, 'index.html', {'form': PastebinForm()})

    def post(self, request):
        form = PastebinForm(request.POST)
        if form.is_valid():
            paste = create_paste(form.cleaned_data['content'])
            return redirect(f'/{paste.key}/')
        else:
            return render(request, 'index.html', {'form': form})


class IndexAPI(View):
    def post(self, request):
        form = PastebinForm(request.POST)
        if form.is_valid():
            paste = create_paste(form.cleaned_data['content'])
            return HttpResponse(f"{request.META['HTTP_HOST']}/{paste.key}/\n")
        else:
            return HttpResponse(repr(form.errors) + '\n', status=400)


class Snippet(View):
    def get(self, request, key):
        snippet = get_object_or_404(Pastebin, key=key)
        style = HtmlFormatter().get_style_defs('.highlight')
        formatter = HtmlFormatter(linenos='inline', wrapcode=True)
        html = highlight(snippet.content, guess_lexer(snippet.content), formatter)
        return render(request, 'code.html', {'code': html, 'style': style, 'key': key})


class RawSnippet(View):
    def get(self, request, key):
        snippet = get_object_or_404(Pastebin, key=key)
        return HttpResponse(snippet.content, content_type="text/plain; charset=utf-8")
