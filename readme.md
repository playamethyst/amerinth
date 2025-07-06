# amerinth

## todo

### features

- auto generate tag enums
- rate limiting
- blocking api
- oauth
- tracing
- utilities
    - mrpack downloader
    - curseforge -> modrinth

### endpoints

- [ ] projects
    - [ ] **GET** `/search`
    - [ ] `project/{id|slug}`
        - [ ] **GET**
        - [ ] **DELETE**
        - [ ] **PATCH**
    - [ ] `/projects`
        - [ ] **GET**
        - [ ] **PATCH**
    - [ ] **GET** `/projects_random`
    - [ ] **POST** `/project`
    - [ ] `/project/{id|slug}/icon`
        - [ ] **DELETE**
        - [ ] **PATCH**
    - [ ] **GET** `/project/{id|slug}/check`
    - [ ] `/project/{id|slug}/gallery`
        - [ ] **POST**
        - [ ] **DELETE**
        - [ ] **PATCH**
    - [ ] **GET** `/project/{id|slug}/dependencies`
    - [ ] `/project/{id|slug}/follow`
        - [ ] **POST**
        - [ ] **DELETE**
    - [ ] `/project/{id|slug}/schedule`
        - [ ] **POST**
- [ ] versions
    - [ ] **GET** `/project/{id|slug}/version`
    - [ ] `/version/{id}`
        - [ ] **GET**
        - [ ] **DELETE**
        - [ ] **PATCH**
    - [ ] **GET** `/project/{id|slug}/version/{id|number}`
    - [ ] **POST** `/version`
    - [ ] **POST** `/version/{id}/schedule`
    - [ ] **GET** `/versions`
    - [ ] **POST** `/version/{id}/file`
- [ ] version-files
    - [ ] `/version_file/{hash}`
        - [ ] **GET**
        - [ ] **DELETE**
    - [ ] **POST** `version_file/{hash}/update`
    - [ ] **GET** `/version_files`
    - [ ] **POST** `/version_files/update` 
- [ ] users
    - [ ] `/user/{id|username}`
        - [x] **GET**
        - [ ] **PATCH**
    - [x] **GET** `/user`
    - [x] **GET** `/users`
    - [ ] `/user/{id|username}/icon`
        - [ ] **DELETE**
        - [ ] **PATCH**
    - [ ] **GET** `/user/{id|username}/projects`
    - [ ] **GET** `/user/{id|username}/follows`
- [ ] notifications
    - [ ] **GET** `/user/{id|username}/notifications`
    - [ ] `/notification/{id}`
        - [ ] **GET**
        - [ ] **DELETE**
        - [ ] **PATCH**
    - [ ] `/notifications`
        - [ ] **GET**
        - [ ] **DELETE**
        - [ ] **PATCH**
- [ ] threads
    - [ ] `/report`
        - [ ] **GET**
        - [ ] **POST**
    - [ ] `/report/{id}`
        - [ ] **GET**
        - [ ] **PATCH**
    - [ ] **GET** `/reports`
    - [ ] `/thread/{id}`
        - [ ] **GET**
        - [ ] **POST**
    - [ ] **GET** `/threads`
    - [ ] **DELETE** `/message/{id}`
- [ ] teams
    - [ ] **GET** `/project/{id|slug}/members`
    - [ ] `/team/{id}/members`
        - [ ] **GET**
        - [ ] **POST**
    - [ ] **GET** `/teams`
    - [ ] **POST** `/team/{id}/join`
    - [ ] `/team/{id}/members/{id|username}`
        - [ ] **DELETE**
        - [ ] **PATCH**
    - [ ] **PATCH** `/team/{id}/owner`
- [ ] tags
    - [x] **GET** `/tag/category`
    - [x] **GET** `/tag/loader`
    - [x] **GET** `/tag/game_version`
    - [x] **GET** `/tag/license/{id}`
    - [ ] **GET** `/tag/donation_platform`
    - [ ] **GET** `/tag/report_type`
    - [ ] **GET** `/tag/project_type`
    - [ ] **GET** `/tag/slide_type`
- [x] misc
    - [x] **GET** `/updates/{id|slug}/forge_updates.json`
    - [x] **GET** `/statistics`

## will not support!

these endpoints are documented but are actually private and not available to the public.

- **GET** `/user/{id|username}/payouts`
- **POST** `/user/{id|username}/payouts`

these endpoints are deprecated, and will therefore not be supported.

- **GET** `/tag/license`