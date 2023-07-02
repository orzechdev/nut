# Nut Revision Control System

Nut - fast, scalable, distributed revision control system. For tracing changes of resources outside of the repository via the files in the repository.

## Examplary use case

### Problem

Teachers and students want to track their learning in the educational course on the website. Students want to get certificate of accomplishment of the course. But course can slightly change each semester on the website.

### Solution

Students can get certificate of accomplishment of the course for the specific version of it (point of time / snapshot), by referencing that version in revision control system which keeps track of resources changes via files in the repository that represents the course.

## Expected project structure

Nut can interprets the following examplary project structure that represents repository definition:

```
|__index.json
|__foo
   |__bar.json
```

The index.json at the root of the project is obligtory. Each new revision will be always named the same as the version in index.json.

### File structure

Each file withing the project (here index.json, bar.json) must contain the following:

```json
{
  "version": "<major.minor.patch>",
  "timestamp": "<Unix-time>",
  "name": "<name>",
  "url": "<url>"
}
```

### Software development files

It is strongly advised to NOT include software withing the same project. Projects versioned with Git should be different projects than those versioned with Nut.

## Nut prototype implementation

> This implementation prototype doesn't implement hashing and doesn't implement tree objects. These drawbacks are introduced to support referencing by semver. If you wish to have hashing, use Git (https://git-scm.com/book/en/v2/Git-Internals-Git-Objects).

1. Create files:

```
|__index.json
|__parts
   |__part-1.json
   |__part-2.json
   |__part-x.json
```

`index.json`, `part-1.json`, `part-2.json`, `part-3.json`:

```json
{
  "version": "1.0.0",
  "timestamp": "<Unix-time>",
  "name": "<name>",
  "url": "<url>"
}
```

2. Initialize the project repository with `nut init` as the shell command:

```
|__index.json
|__parts
|  |__part-1.json
|  |__part-2.json
|  |__part-x.json
|
|  # Newly created Nut directory
|__.nut
   |__nut.json
```

`nut.json`:

```json
{
  "nut-api": "<nut-api-version>",
  "head": null
}
```

3. Add the project snapshot, execute `nut commit`:

```
|__index.json
|__parts
|  |__part-1.json
|  |__part-2.json
|  |__part-x.json
|
|  # Newly created Nut directory
|__.nut
   |__nut.json
   |__revisions
   |  |__1.0.0.json
   |
   |  # Compressed files named with SHA-1 checksum
   |__blobs
      |__ce3e34c9fb1ceef0bb37ae35971f1f0a4a603333 # index.json @1.0.0
      |__064f4e9968c049c5af6648a95e0bf4db89e0a578 # part-1.json @1.0.0
      |__9a230a20d267dac0b8a7eb771429e58f12a67360 # part-2.json @1.0.0
      |__c11cc66a5f08acf78abd4b8999e8fa1af395ab4c # part-x.json @1.0.0
```

`revisions/1.0.0.json`:

```json
{
  "blobs": {
    "index": "ce3e34c9fb1ceef0bb37ae35971f1f0a4a603333", // @1.0.0
    "parts": {
      "part-1": "064f4e9968c049c5af6648a95e0bf4db89e0a578", // @1.0.0
      "part-2": "9a230a20d267dac0b8a7eb771429e58f12a67360", // @1.0.0
      "part-x": "c11cc66a5f08acf78abd4b8999e8fa1af395ab4c" // @1.0.0
    }
  },
  "parent": null
}
```

`nut.json`:

```json
{
  "nut-api": "<nut-api-version>",
  "head": "1.0.0"
}
```

4. Modify files, update in each version to 1.0.1.

5. Execute another `nut commit`:

```
|__index.json
|__parts
|  |__part-1.json
|  |__part-2.json
|  |__part-x.json
|
|  # Newly created Nut directory
|__.nut
   |__nut.json
   |__revisions
   |  |__1.0.0.json
   |  |__1.0.1.json
   |
   |  # Compressed files named with SHA-1 checksum
   |__blobs
      |__ce3e34c9fb1ceef0bb37ae35971f1f0a4a603333 # index.json @1.0.0
      |__064f4e9968c049c5af6648a95e0bf4db89e0a578 # part-1.json @1.0.0
      |__9a230a20d267dac0b8a7eb771429e58f12a67360 # part-2.json @1.0.0
      |__c11cc66a5f08acf78abd4b8999e8fa1af395ab4c # part-x.json @1.0.0
      |__071fdc09eaf954917c467f2e143a29b3e82b3e7b # index.json @1.0.1
      |__92f115f2abb60d990faaf99deacbad01102b0fd9 # part-1.json @1.0.1
      |__ff5ec5b6f107947be45b5e0e59822faf1985721e # part-2.json @1.0.1
      |__31358c76254969e81e71ceba2a0453489dd4ab45 # part-x.json @1.0.1
```

`revisions/1.0.1.json`:

```json
{
  "blobs": {
    "index": "071fdc09eaf954917c467f2e143a29b3e82b3e7b", // @1.0.1
    "parts": {
      "part-1": "92f115f2abb60d990faaf99deacbad01102b0fd9", // @1.0.1
      "part-2": "ff5ec5b6f107947be45b5e0e59822faf1985721e", // @1.0.1
      "part-x": "31358c76254969e81e71ceba2a0453489dd4ab45" // @1.0.1
    }
  },
  "parent": "1.0.0"
}
```

`nut.json`:

```json
{
  "nut-api": "<nut-api-version>",
  "head": "1.0.1"
}
```

The .nut/revisions directory contains files with snapshots structured with linked list data structure starting from head from nut.json. The revision is always named the same as the version in index.json.

To create a revision, first it is needed to bump index.json version.

## Differences between Nut and Git

- Git tracks changes performed in the files in the repository. Nut tracks changes performed in the resources outside of the repository via files in the repository.
- The other party can reference the revision from the Nut repositories easily with `<repository-name>@<major.minor.patch>`. Git lets to use hash when referencing commits instead of semver. Nut for `<repository-name>@1.0.0` will return all files from revision equals to 1.0.0.
