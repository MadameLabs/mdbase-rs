---
kind: mdbase.type
name: person
version: 1
description: Pessoa da base.
match:
  where:
    type: person
schema:
  dialect: json-schema-2020-12
  value:
    $schema: https://json-schema.org/draft/2020-12/schema
    type: object
    required:
    - name
    additionalProperties: true
    properties:
      type:
        const: person
      name:
        type: string
        minLength: 1
      identities:
        type: array
        uniqueItems: true
        description: Editable identity associations. Missing or empty means this person
          has no linked account. Match issuer and subject exactly; never match by
          email or name.
        items:
          type: object
          required:
          - issuer
          - subject
          properties:
            issuer:
              type: string
              format: uri
              pattern: ^https?://
              description: Stable issuer URL supplied by the identity service. Copy
                exactly; do not derive from a collection authority or relay URL.
            subject:
              type: string
              pattern: \S
              description: Opaque, case-sensitive, account-wide identifier supplied
                by the issuer. Not an email, membership ID, or credential.
          additionalProperties: false
      role:
        type: string
      email:
        type: string
        format: email
implements:
- contract: mdbase.person
  version: 2.0.0
  fields:
    name: name
    identities: identities
    bogus: x
---
