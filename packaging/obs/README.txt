OBS quickstart for catopy

1) Install osc:
   sudo dnf install osc

2) Checkout your OBS project/package:
   osc checkout home:febrezo catopy

3) Copy catopy.spec and sources metadata into the OBS package directory.

4) Local build test:
   osc build openSUSE_Tumbleweed x86_64 catopy.spec

5) Commit:
   osc commit

Notes:
- Keep catopy.spec in repository root as the canonical spec.
- Adjust dependencies/macros per target distribution if OBS reports issues.
