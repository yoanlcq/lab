#!/bin/bash

# TODO actually test this with files

user=lecoqy
site=yoanlecoq.com

lftp $site -u $user -e " \
    set ssl:verify-certificate no; \
    set ftp:list-options -a; \
    mirror --only-missing --only-newer --parallel=3 --loop --verbose=3; \
"
