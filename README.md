
# Clock Alert

It's not a surprise if I tell you that waking up on time can sometimes be hard. Introducing Clock Alert, a revolutionary, brand-new tool that has never been utilized before. With Clock Alert, you can set an exact time, and when that time arrives, music will play. It's incredible, amazing, outstanding!


## Technical environment

#### Tauri application:

- Package name: clockalert
- Language use in frontend: TypeScript
- Package manager · npm
- UI template · React - (https://react.dev/)
- UI flavor · TypeScript

#### Others

- DB Sqlite
- Material UI


## Authors

- [@CedKLEIN](https://www.github.com/CedKLEIN)


## Deployment

To deploy this project run

```bash
  npm install
  chmod +x ./src/scripts/install-missing-deps.sh
  ./src/scripts/install-missing-deps.sh
  npm run tauri dev
```


## Running Tests

To run tests, run the following command

```bash
  cd src-tauri
  cargo test
```


## Features

- Clock live
- Alerts
- Catchy music


## Possible improvements

- Front-end unit tests
    - https://tauri.app/fr/v1/guides/testing/mocking/
- Better UI
- Icon update
- Pop up for user error
    - unique alerts
- Make the app async


## Screenshots

![Capture d'écran 2024-06-30 183815](https://github.com/CedKLEIN/ClockAlert/assets/66410614/0be61d3e-8be0-411d-9e37-7c8c0a87f6cc)


## 🔗 References

https://www.npmjs.com/package/react-clock

https://docs.rs/sqlite/latest/sqlite/

https://www.npmjs.com/package/react-clock?activeTab=readme

https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests

https://readme.so/fr/editor

https://github.com/RandomEngy/tauri-sqlite/blob/main/src-tauri/src/database.rs

https://github.com/RandomEngy/tauri-sqlite/blob/main/src-tauri/src/main.rs

https://codesandbox.io/s/notification-sound-react-nn7tk?file=/src/App.js

https://pixabay.com/fr/music/search/alarm/

https://www.reddit.com/r/learnjavascript/comments/108saqh/property_play_does_not_exist_on_type_never/

https://outils-javascript.aliasdmc.fr/encodage-caracteres-formulaire/encode-caractere-274C-html-css-js-autre.html

https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html

