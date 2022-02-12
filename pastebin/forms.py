from django import forms


class PastebinForm(forms.Form):
    content = forms.CharField(
        max_length=1024 * 512,
        widget=forms.Textarea(attrs={'style': 'width: 100%; ', 'cols': ''}),
    )
