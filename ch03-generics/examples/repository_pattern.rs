//! ジェネリックな Repository トレイトと InMemory 実装。
//!
//! 関連型 (Entity, Id, Error) を使うと、利用側は型パラメータを並べずに
//! `R: Repository` だけで全ての制約を巻き取れる。Ch.2 で扱った
//! 「関連型 vs ジェネリクス」の判断軸を、ライブラリ設計の文脈で適用した例。

use std::collections::HashMap;
use std::hash::Hash;

pub trait Repository {
    type Entity;
    type Id: Eq + Hash + Clone;
    type Error: std::fmt::Debug;

    fn find(&self, id: &Self::Id)
        -> Result<Option<Self::Entity>, Self::Error>;
    fn save(&mut self, id: Self::Id, entity: Self::Entity)
        -> Result<(), Self::Error>;
}

#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
}

#[derive(Default)]
pub struct InMemoryUserRepo {
    store: HashMap<u64, User>,
}

#[derive(Debug)]
pub struct RepoError(#[allow(dead_code)] String);

impl Repository for InMemoryUserRepo {
    type Entity = User;
    type Id = u64;
    type Error = RepoError;

    fn find(&self, id: &u64) -> Result<Option<User>, RepoError> {
        Ok(self.store.get(id).cloned())
    }

    fn save(&mut self, id: u64, user: User) -> Result<(), RepoError> {
        if user.name.is_empty() {
            return Err(RepoError("name is empty".into()));
        }
        self.store.insert(id, user);
        Ok(())
    }
}

// 利用側は R: Repository だけで全制約を引き連れる。
fn upsert<R: Repository<Id = u64>>(
    repo: &mut R,
    id: u64,
    entity: R::Entity,
) -> Result<(), R::Error> {
    repo.save(id, entity)
}

fn main() {
    let mut repo = InMemoryUserRepo::default();
    upsert(&mut repo, 1, User { name: "alice".into() }).unwrap();
    let found = repo.find(&1).unwrap();
    println!("found: {found:?}");
    let err = upsert(&mut repo, 2, User { name: "".into() });
    println!("err  : {err:?}");
}
