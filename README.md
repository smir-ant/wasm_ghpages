## Стек
- [WebAssebmlhy (wasm)](https://habr.com/ru/articles/475778/) через rust
- [Leptos](https://leptos.dev/)
- [Tailwind](https://tailwindcss.ru/)
- [Github Pages](https://docs.github.com/ru/pages/getting-started-with-github-pages/about-github-pages)
- [Github Actions](https://doka-guide.vercel.app/tools/github-actions/)

# Начать работу в двух шагах:
1) Скачиваем
2) `trunk serve --open` в терминал

# Action
Уже прописан на автоматическую работу.
Суть: прилетело обновление репозитория? будет запущена сборка, от установки rust до перебрасывание получаемых статических файлов(html, js, wasm, css, favicon) в ветку `gh-pages`.

# Tailwind
Встроен в leptos, ничего не нужно устанавливать дополнительно, само собирается. Главное наличие `rel="tailwind-css"`.

Если хочется отключить tailwind, то убираем подключение (`rel="tailwind-css"`) в html; убираем классы у body в html; удаляем `tailwind.css`.

Если важен флаг `--minify`, то загляни в `Trunk.toml`.

К сожалению, общее время от пуша обновления до обновленной страницы ~6 минут. <br>Trunk долго компилится и проект собирается также... Пока не вдуплил как ускорить.

---

# Буду рад issues и звёздам