# Ichor

An API wrapper for [itch.io](https://itch.io)

## Notes

If you ever want an endpoint to be added, just [open an issue](issues/new)!

Itch.io's [API reference](https://itch.io/docs/api/serverside) is not great. A bunch of endpoints are missing/undocumented.

Because no OpenAPI spec exists, a lot of the data models could be incomplete. If you ever run into issues, please [open an issue](issues/new) with the correct data you received from the endpoint. 

## Contributing

If you want to PR something, [quicktype](https://app.quicktype.io/) is useful to generate data models.

However, please replace `Vec<T>` with the provided `MaybeEmptyList<T>` if applicable- the API sometimes returns `{}` in place of an empty list. 

Preferably, also make sure if things are `Option` or actually required with a blank account.
