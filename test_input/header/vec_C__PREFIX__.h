/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   vec_C__PREFIX__.h                                  :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: maiboyer <maiboyer@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/04 18:46:53 by maiboyer          #+#    #+#             */
/*   Updated: 2023/12/07 19:17:40 by maiboyer         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef VEC_C__PREFIXUP___H
#define VEC_C__PREFIXUP___H

#ifdef GENERIC_WRITING
#define C__TYPENAME__ char
#define C__TYPEHEADER__
#endif

C__TYPEHEADER__
#include "me/types.h"

typedef struct {
  t_usize len;
  t_usize capacity;
  C__TYPENAME__ *buffer;
} t_vec_C__PREFIX__;

t_vec_C__PREFIX__ me_vec_C__PREFIX___new(t_usize capacity);
bool me_vec_C__PREFIX___push(t_vec_C__PREFIX__ *vec, C__TYPENAME__ element);
bool me_vec_C__PREFIX___pop(t_vec_C__PREFIX__ *vec, C__TYPENAME__ *value);
void me_vec_C__PREFIX___free(t_vec_C__PREFIX__ vec,
                             void (*free_elem)(C__TYPENAME__));

#endif
