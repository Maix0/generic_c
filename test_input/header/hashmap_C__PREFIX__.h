/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   hashmap_C__PREFIX__.h                              :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: maiboyer <maiboyer@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/06 11:00:22 by maiboyer          #+#    #+#             */
/*   Updated: 2023/12/07 21:47:17 by maix             ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef HASHMAP_C__PREFIXUP___H
#define HASHMAP_C__PREFIXUP___H

#define DEFAULT_BUCKETS 750

#include "me/types.h"
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifdef GENERIC_WRITING
#define C__KEYTYPE__ char *
#define C__VALTYPE__ char *
#endif

typedef struct s_kv_C__PREFIX__ {
  C__KEYTYPE__ key;
  C__VALTYPE__ val;
} t_kv_C__PREFIX__;

typedef size_t (*t_hash_C__PREFIX___fn)(C__KEYTYPE__ *key);
typedef size_t (*t_drop_C__PREFIX___fn)(t_kv_C__PREFIX__ val);
typedef bool (*t_eq_C__PREFIX___fn)(C__KEYTYPE__ *lhs, C__KEYTYPE__ *rhs);

typedef struct s_entry {
  size_t hash_id;
  t_kv_C__PREFIX__ kv;
  struct s_entry_C__PREFIX__ *next;
} t_entry_C__PREFIX__;

typedef struct s_hashmap_C__PREFIX__ {
  t_entry_C__PREFIX__ **buckets;
  size_t num_buckets;
  t_hash_C__PREFIX___fn hfunc;
  t_eq_C__PREFIX___fn cfunc;
  t_drop_C__PREFIX___fn drop;
} t_hashmap_C__PREFIX__;

t_hashmap_C__PREFIX__ *new_hashmap(t_hash_C__PREFIX___fn hash, t_eq_C__PREFIX___fn cmp, t_drop_C__PREFIX___fn drop);
t_hashmap_C__PREFIX__ *new_hashmap_c(t_hash_C__PREFIX___fn hash, t_eq_C__PREFIX___fn cmp, t_drop_C__PREFIX___fn drop,
                         size_t cap);

void drop_hashmap(t_hashmap_C__PREFIX__ hmap);

void insert_hashmap_C__PREFIX__(t_hashmap_C__PREFIX__ *hmap, C__KEYTYPE__ key, C__PREFIX__ value);

C__PREFIX__ *get_hashmap_C__PREFIX__(t_hashmap_C__PREFIX__ *hmap, C__KEYTYPE__ *key);
void remove_hashmap(t_hashmap_C__PREFIX__ *hmap, C__KEYTYPE__ *key);

#endif
