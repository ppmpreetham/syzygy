use rayon::prelude::*;

fn sequential(){
  todo!("Sequantially attacks, one after another")
}

fn parallel(lists: &[&[u8]]){
  todo!("parallelly attacks, with the same group/bucket")
}

fn matrix(lists: &[&[u8]]){
  let combinations: usize = lists.iter().map(|l| l.len()).product();

  let mut strides = vec![1; lists.len()];
  for i in (0..lists.len() - 1).rev() {
      strides[i] = strides[i + 1] * lists[i + 1].len();
  }

  (0..combinations).into_par_iter().for_each(|idx| {
      let mut combo = vec![0u8; lists.len()];

      let mut curr_idx = idx;
      for i in 0..lists.len() {
          let list_idx = curr_idx / strides[i];
          combo[i] = lists[i][list_idx];
          curr_idx %= strides[i];
      }

      todo!("send the req here");
  });
}
