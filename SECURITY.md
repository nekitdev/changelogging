# Security Policy

## Reporting

Thank you for taking the time to responsibly disclose any problems you find.

**Do not file public issues as they are open for everyone to see!**

All security vulnerabilities in `changelogging` should be reported by email
to [security@nekit.dev][Security Email].
Your report will be acknowledged within 24 hours, and you will receive a more
detailed response within 48 hours indicating the next steps in handling your report.

You can encrypt your report using our public key:
[`EEBD6848A583A1B4F6B73B60D177918FC00BB74F`][Security Key].
This key is also available on [MIT's Key Server][MIT Key Server]
and [reproduced below](#security-key).

After the initial reply to your report, the core team will try to keep you
informed of the progress being made towards a fix and official announcement.
These updates will be sent at least every five days. In reality, this is
more likely to be every 24-48 hours.

## Disclosure Policy

`changelogging` has a 5-step disclosure process:

1. The security report is received and is assigned a primary handler.
   This person will coordinate the fix and release process.

2. The problem is confirmed and a list of all affected versions is determined.

3. Code is audited to find any potential similar problems.

4. Fixes are prepared for all releases which are still under maintenance.
   These fixes are not committed to the public repository but rather
   held locally pending the announcement.

5. On the embargo date, the changes are pushed to the public repository
   and new builds are deployed.

This process can take some time, especially when coordination is required
with maintainers of other projects. Every effort will be made to handle
the issue in as timely a manner as possible, however it is important that
we follow the release process above to ensure that the disclosure is handled
in a consistent manner.

## Security Key

```text
-----BEGIN PGP PUBLIC KEY BLOCK-----

mDMEac+/LRYJKwYBBAHaRw8BAQdAxZdnBe5EWI4ZV33a9MpRdNtkB3c02mMriiV9
Cv88tQa0L05pa2l0YSBUaWtob25vdiAoc2VjdXJpdHkpIDxzZWN1cml0eUBuZWtp
dC5kZXY+iJAEExYKADgWIQTuvWhIpYOhtPa3O2DRd5GPwAu3TwUCac+/LQIbAwUL
CQgHAgYVCgkICwIEFgIDAQIeAQIXgAAKCRDRd5GPwAu3T6iHAQDe0Q+0rjKMg2rf
TZ1lHXeKmIg1EW8ZB+W/z05ahnHpGwD5AbR0fCxiM5IMqHaEdwJl6fxE9TYDvM7g
NWoRdbiHIAe4OARpz78tEgorBgEEAZdVAQUBAQdAfGJto7sXSt0YbBUDkK901tpA
m08yaiqDJ0MgsytMFkgDAQgHiHgEGBYKACAWIQTuvWhIpYOhtPa3O2DRd5GPwAu3
TwUCac+/LQIbDAAKCRDRd5GPwAu3T4QBAP9v6yLLf9FVAnD3XNGzz9juTk/z+sSL
F9XiwmRXQJ31SQEA/QmBRPc35EFA+zA+wwx8beeeg6A8iYABIpXde15Xgw8=
=fKBT
-----END PGP PUBLIC KEY BLOCK-----
```

## Attribution

This Security Policy is adapted from [Rust's Security Policy][Rust Security Policy].

[Security Email]: mailto:security@nekit.dev
[Security Key]: https://nekit.dev/keys/security
[MIT Key Server]: https://pgp.mit.edu/pks/lookup?op=index&search=0xEEBD6848A583A1B4F6B73B60D177918FC00BB74F
[Rust Security Policy]: https://rust-lang.org/policies/security
'
