/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   hashmap_C__PREFIX__.c                              :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: maiboyer <maiboyer@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/06 10:58:20 by maiboyer          #+#    #+#             */
/*   Updated: 2023/12/07 21:37:23 by maix             ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "me/hashmap/hashmap_C__PREFIX__.h"
#include "me/mem/calloc.h"
#include "me/mem/malloc.h"
#include "me/mem/memcpy.h"
#include "me/types.h"
#include <stdlib.h>

t_hashmap_C__PREFIX__ *new_C__PREFIX___hashmap(t_hash_C__PREFIX___fn hfunc, t_eq_C__PREFIX___fn cfunc, t_drop_C__PREFIX___fn drop)
{
	return (new_hashmap_C__PREFIX___with_capacity(hfunc, cfunc, drop, DEFAULT_BUCKETS));
}

t_hashmap_C__PREFIX__ *new_hashmap_C__PREFIX___with_capacity(t_hash_C__PREFIX___fn hfunc, t_eq_C__PREFIX___fn cfunc, t_drop_C__PREFIX___fn drop,
						 size_t buckets)
{
	t_hashmap_C__PREFIX__ *hmap;

	hmap = malloc(sizeof(*hmap));
	if (hmap == NULL)
		return (NULL);
	hmap->buckets = me_calloc(buckets, sizeof(t_entry_C__PREFIX__ *));
	hmap->num_buckets = buckets;
	hmap->hfunc = hfunc;
	hmap->cfunc = cfunc;
	hmap->drop = drop;
	if (hmap->buckets == NULL)
		return ((void)free(hmap), NULL);
	return (hmap);
}

void drop_C__PREFIX___hashmap(t_hashmap_C__PREFIX__ hmap)
{
	t_usize index;

	index = 0;
	while (index < hmap.num_buckets)
	{
		if (hmap.buckets[index])
		{
			hmap.drop(hmap.buckets[index]->kv);
			free(hmap.buckets[index]);
		}
		index++;
	}
	free(hmap.buckets);
}
